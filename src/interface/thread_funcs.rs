use std::collections::HashMap;
use std::{io, sync::Arc};
use std::sync::{Mutex, mpsc};

use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use pnet::packet::{Packet, MutablePacket};

use crate::napt::NAPTer;
use crate::scheduler::RoundRobinScheduler;
use crate::crypto;

use super::lan::LanInput;
use super::wan::WanOutput;

pub fn lan_thread_func(
  lan_in: &mut LanInput,
  wan_out: &mut Vec<WanOutput>,
  napter: Arc<Mutex<NAPTer>>
) -> io::Result<()> {
  let mut scheduler = RoundRobinScheduler::new(wan_out.len());
  let mut ip_id = 1u16;
  loop {
    let received_buf = lan_in.rx.next()?;
    let mut ether_frame0 = MutableEthernetPacket::owned(received_buf.to_vec()).unwrap();
    if ether_frame0.get_ethertype() == EtherTypes::Ipv4 {
      let mut ether_frame1 = MutableEthernetPacket::owned(ether_frame0.packet().to_vec()).unwrap();
      let out_intf0 = &mut wan_out[scheduler.next()];
      let out_intf1 = &mut wan_out[scheduler.next()];
      let (key, iv) = crypto::load_crypto_info()?;
      {
        let mut ip_packet0 = MutableIpv4Packet::new(ether_frame0.payload_mut()).unwrap();
        let mut ip_packet1 = MutableIpv4Packet::new(ether_frame1.payload_mut()).unwrap();
        let payload0 = crypto::encrypt_packet(&received_buf[..received_buf.len()/2], &key, &iv)?;
        let payload1 = crypto::encrypt_packet(&received_buf[received_buf.len()/2..], &key, &iv)?;
        ip_packet0.set_payload(&payload0);
        ip_packet1.set_payload(&payload1);
        ip_packet0.set_identification(ip_id);
        ip_packet1.set_destination(ip_id);
        ip_id += 1;
        out_intf0.ip_encap(&mut ip_packet0);
        out_intf1.ip_encap(&mut ip_packet1);
        ip_packet1.set_fragment_offset((received_buf.len()/2) as u16);
      }
      out_intf0.ether_encap(&mut ether_frame0);
      out_intf1.ether_encap(&mut ether_frame1);
      out_intf0.send(ether_frame0.packet());
      out_intf1.send(ether_frame1.packet());
      // if let Some(napted_packet) = napter.translate_outgoing(ip_packet) {
      //   if let Some(ether_to_send) = out_intf.ether_encap(napted_packet) {
      //     out_intf.send(ether_to_send)?;
      //   };
      // };
    };
  }
}

pub fn wan_intf_func(
  wan_in: &mut IntfInput,
  lan_out_arc: Arc<Mutex<IntfOutput>>,
  napter: Arc<Mutex<NAPTer>>
) -> io::Result<()> {
  loop {
    let ether_frame = wan_in.receive()?;
    if let Some(ip_packet) = wan_in.classify(ether_frame) {
      let napter = napter.lock().unwrap();
      if let Some(napted_packet) = napter.translate_incoming(ip_packet) {
        let mut lan_out = lan_out_arc.lock().unwrap();
        if let Some(ether_to_send) = lan_out.ether_encap(napted_packet) {
          lan_out.send(ether_to_send)?;
        };
      };
    };
  }
}

pub fn wan_bounding_func(
  wan_channels: &Vec<Box<mpsc::Receiver<Vec<u8>>>>
) {
  let mut fragment_map: HashMap<u16, Vec<u8>> = HashMap::new();
  let (key, iv) = crypto::load_crypto_info()?;
  loop {
    for channel in wan_channels {
      match channel.try_recv() {
        Ok(packet) => {
          let ether_frame = EthernetPacket::owned(packet).unwrap();
          let ip_packet = Ipv4Packet::new(ether_frame.payload()).unwrap();
          let content = decrypt_packet(ip_packet.payload(), key, iv);
        },
        Err(mpsc::TryRecvError::Empty) => {},
        Err(mpsc::TryRecvError::Disconnected) => return,
      }
    }
  }
}
