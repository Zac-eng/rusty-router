use std::io;
use std::collections::HashMap;
use std::sync::mpsc;

use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use pnet::packet::{MutablePacket, Packet};

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
  let (key, iv) = crypto::load_crypto_info()?;
  loop {
    let received_buf = lan_in.rx.next()?;
    let mut ether_frame = MutableEthernetPacket::owned(received_buf.to_vec()).unwrap();
    let ip_buf_len = ether_frame.payload().len();
    if ether_frame.get_ethertype() == EtherTypes::Ipv4 {
      {
        let out_intf = &mut (wan_out[scheduler.next()]);
        {
          let mut ip_packet = MutableIpv4Packet::new(ether_frame.payload_mut()).unwrap();
          let payload = crypto::encrypt_packet(&ip_packet.packet()[..ip_buf_len/2], &key, &iv)?;
          // let payload = &received_buf[..buf_len/2];
          ip_packet.set_payload(&payload);
          ip_packet.set_identification(ip_id);
          out_intf.ip_encap(&mut ip_packet);
        }
        out_intf.ether_encap(&mut ether_frame);
        match out_intf.send(ether_frame.packet()) {
          Ok(_) => {},
          Err(_) => return Err(io::Error::new(io::ErrorKind::BrokenPipe, "wan channel"))
        }
      }
      {
        let out_intf = &mut (wan_out[scheduler.next()]);
        {
          let mut ip_packet = MutableIpv4Packet::new(ether_frame.payload_mut()).unwrap();
          let payload = crypto::encrypt_packet(&ip_packet.packet()[ip_buf_len/2..], &key, &iv)?;
          // let payload = &received_buf[buf_len/2..];
          ip_packet.set_payload(&payload);
          ip_packet.set_identification(ip_id);
          out_intf.ip_encap(&mut ip_packet);
          ip_packet.set_fragment_offset((ip_buf_len/2) as u16);
        }
        out_intf.ether_encap(&mut ether_frame);
        match out_intf.send(ether_frame.packet()) {
          Ok(_) => {},
          Err(_) => return Err(io::Error::new(io::ErrorKind::BrokenPipe, "wan channel"))
        }
      }
      ip_id += 1;
    };
  }
}

pub fn wan_intf_func(
  wan_in: &mut WanInput,
  wan_channel: &mpsc::Sender<Vec<u8>>
) -> io::Result<()> {
  loop {
    let mut ether_frame = wan_in.receive()?;
    if let Some(ip_packet) = wan_in.classify(&mut ether_frame) {
      match wan_channel.send(ip_packet.packet().to_vec()) {
        Ok(_) => continue,
        Err(_) => return Err(io::Error::new(io::ErrorKind::BrokenPipe, "wan channel"))
      }
    };
  }
}

pub fn wan_bounding_func(
  lan_out: &mut LanOutput,
  wan_channels: &Vec<mpsc::Receiver<Vec<u8>>>
) -> io::Result<()> {
  let mut fragment_map: HashMap<u16, Vec<u8>> = HashMap::new();
  let (key, iv) = crypto::load_crypto_info()?;
  loop {
    for channel in wan_channels {
      match channel.try_recv() {
        Ok(packet) => {
          let ip_packet = Ipv4Packet::owned(packet).unwrap();
          let content = crypto::decrypt_packet(ip_packet.payload(), &key, &iv)?;
          // let content = ip_packet.payload();
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
            match lan_out.send(lan_out.ether_encap(original_packet).unwrap()) {
              Ok(_) => continue,
              Err(_) => return Err(io::Error::new(io::ErrorKind::BrokenPipe, "wan channel"))
            }
          } else {
            fragment_map.insert(id, content.to_vec());
          }
        },
        Err(mpsc::TryRecvError::Empty) => {},
        Err(mpsc::TryRecvError::Disconnected) => return Err(io::Error::new(io::ErrorKind::ConnectionAborted, "wan channel")),
      }
    }
  }
}
