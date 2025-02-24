use std::collections::HashMap;
use std::{io, sync::Arc};
use std::sync::{Mutex, mpsc};

use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;

use crate::napt::NAPTer;
use crate::scheduler::RoundRobinScheduler;
use crate::crypto;
use super::{IntfInput, IntfOutput};

pub fn lan_thread_func(
  lan_in: &mut IntfInput,
  wan_out: &mut Vec<IntfOutput>,
  napter: Arc<Mutex<NAPTer>>
) -> io::Result<()> {
  let mut scheduler = RoundRobinScheduler::new(wan_out.len());
  loop {
    let ether_frame = lan_in.receive()?;
    if let Some(ip_packet) = lan_in.classify(ether_frame) {
      let out_intf = &mut wan_out[scheduler.next()];
      let mut napter = napter.lock().unwrap();
      if let Some(napted_packet) = napter.translate_outgoing(ip_packet) {
        if let Some(ether_to_send) = out_intf.ether_encap(napted_packet) {
          out_intf.send(ether_to_send)?;
        };
      };
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
