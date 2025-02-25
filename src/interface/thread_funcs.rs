use std::collections::HashMap;
use std::{io, sync::Arc};
use std::sync::{Mutex, mpsc};

use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use pnet::packet::{self, MutablePacket, Packet};

use crate::napt::NAPTer;
use crate::scheduler::RoundRobinScheduler;
use crate::crypto;

use super::lan::{LanInput, LanOutput};
use super::wan::{WanInput, WanOutput};

pub fn lan_thread_func(
  lan_in: &mut LanInput,
  wan_out: &mut Vec<WanOutput>,
) -> io::Result<()> {
  let mut scheduler = RoundRobinScheduler::new(wan_out.len());
  let mut ip_id = 1u16;
  loop {
    let received_buf = lan_in.rx.next()?;
    let mut ether_frame = MutableEthernetPacket::owned(received_buf.to_vec()).unwrap();
    let buf_len = received_buf.len();
    if ether_frame.get_ethertype() == EtherTypes::Ipv4 {
      let (key, iv) = crypto::load_crypto_info()?;
      {
        let out_intf = &mut (wan_out[scheduler.next()]);
        {
          let mut ip_packet = MutableIpv4Packet::new(ether_frame.payload_mut()).unwrap();
          let payload = crypto::encrypt_packet(&received_buf[..buf_len/2], &key, &iv)?;
          ip_packet.set_payload(&payload);
          ip_packet.set_identification(ip_id);
          out_intf.ip_encap(&mut ip_packet);
        }
        out_intf.ether_encap(&mut ether_frame);
        out_intf.send(ether_frame.packet());
      }
      {
        let out_intf = &mut (wan_out[scheduler.next()]);
        {
          let mut ip_packet = MutableIpv4Packet::new(ether_frame.payload_mut()).unwrap();
          let payload = crypto::encrypt_packet(&received_buf[buf_len/2..], &key, &iv)?;
          ip_packet.set_payload(&payload);
          ip_packet.set_identification(ip_id);
          out_intf.ip_encap(&mut ip_packet);
          ip_packet.set_fragment_offset((buf_len/2) as u16);
        }
        out_intf.ether_encap(&mut ether_frame);
        out_intf.send(ether_frame.packet());
      }
      ip_id += 1;
    };
  }
}

pub fn wan_intf_func(
  wan_in: &mut WanInput,
  wan_channel: &Box<mpsc::Sender<Vec<u8>>>
) -> io::Result<()> {
  loop {
    let mut ether_frame = wan_in.receive()?;
    if let Some(ip_packet) = wan_in.classify(&mut ether_frame) {
      wan_channel.send(ip_packet.packet().to_vec());
    };
  }
}

pub fn wan_bounding_func(
  lan_out: &mut LanOutput,
  wan_channels: &Vec<Box<mpsc::Receiver<Vec<u8>>>>
) -> io::Result<()> {
  let mut fragment_map: HashMap<u16, Vec<u8>> = HashMap::new();
  let (key, iv) = crypto::load_crypto_info()?;
  loop {
    for channel in wan_channels {
      match channel.try_recv() {
        Ok(packet) => {
          let ip_packet = Ipv4Packet::owned(packet).unwrap();
          let content = crypto::decrypt_packet(ip_packet.payload(), &key, &iv)?;
          let id = ip_packet.get_identification();
          if let Some(another) = fragment_map.remove(&id) {
            let mut packet_buf: Vec<u8> = Vec::new();
            match ip_packet.get_fragment_offset() {
              0 => {
                packet_buf.extend(content);
                packet_buf.extend(another);
              },
              _ => {
                packet_buf.extend(another);
                packet_buf.extend(content);
              }
            }
            let original_packet = MutableIpv4Packet::owned(packet_buf).unwrap();
            lan_out.send(lan_out.ether_encap(original_packet).unwrap());
          } else {
            fragment_map.insert(id, content);
          }
        },
        Err(mpsc::TryRecvError::Empty) => {},
        Err(mpsc::TryRecvError::Disconnected) => return Err(io::Error::new(io::ErrorKind::ConnectionAborted, "wan channel")),
      }
    }
  }
}
