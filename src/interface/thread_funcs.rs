use std::io;
use std::collections::HashMap;
use std::sync::mpsc;

use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use pnet::packet::Packet;

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
    let mut buffer = received_buf.to_vec();
    let received_ether = EthernetPacket::new(&received_buf).unwrap();
    let received_ip = Ipv4Packet::new(received_ether.payload()).unwrap();
    let ihl = received_ip.get_header_length() as usize * 4;
    if ihl < 20 {continue;}
    let ip_buf_len = received_ip.packet().len();
    if received_ether.get_ethertype() == EtherTypes::Ipv4 && received_ip.get_destination() != lan_in.ipv4_addr && received_ip.get_source() != lan_in.ipv4_addr {
      {
        let out_intf = &mut (wan_out[scheduler.next()]);
        {
          println!("sent: {:?}", received_ip.packet());
          let payload = crypto::encrypt_packet(&received_ip.packet()[..ip_buf_len/2], &key, &iv)?;
          buffer.resize(14+ihl+payload.len(), 0);
          let mut ip_packet = MutableIpv4Packet::new(&mut buffer[14..]).unwrap();
          ip_packet.set_total_length(ip_packet.packet().len() as u16);
          ip_packet.set_payload(&payload);
          ip_packet.set_identification(ip_id);
          out_intf.ip_encap(&mut ip_packet);
        }
        let mut ether_frame = MutableEthernetPacket::new(&mut buffer).unwrap();
        out_intf.ether_encap(&mut ether_frame);
        match out_intf.send(ether_frame.packet()) {
          Ok(_) => {},
          Err(_) => return Err(io::Error::new(io::ErrorKind::BrokenPipe, "wan channel"))
        }
      }
      {
        let out_intf = &mut (wan_out[scheduler.next()]);
        {
          let payload = crypto::encrypt_packet(&received_ip.packet()[ip_buf_len/2..], &key, &iv)?;
          buffer.resize(14+ihl+payload.len(), 0);
          let mut ip_packet = MutableIpv4Packet::new(&mut buffer[14..]).unwrap();
          ip_packet.set_total_length(ip_packet.packet().len() as u16);
          ip_packet.set_payload(&payload);
          ip_packet.set_identification(ip_id);
          ip_packet.set_fragment_offset((ip_buf_len/2) as u16);
          out_intf.ip_encap(&mut ip_packet);
        }
        let mut ether_frame = MutableEthernetPacket::new(&mut buffer).unwrap();
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
          if let Ok(content) = crypto::decrypt_packet(ip_packet.payload(), &key, &iv) {
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
              println!("concated: {:?}", packet_buf);
              let original_packet = MutableIpv4Packet::owned(packet_buf).unwrap();
              match lan_out.send(lan_out.ether_encap(original_packet).unwrap()) {
                Ok(_) => continue,
                Err(_) => return Err(io::Error::new(io::ErrorKind::BrokenPipe, "wan channel"))
              }
            } else {
              fragment_map.insert(id, content.to_vec());
            }
          }
        },
        Err(mpsc::TryRecvError::Empty) => {},
        Err(mpsc::TryRecvError::Disconnected) => return Err(io::Error::new(io::ErrorKind::ConnectionAborted, "wan channel")),
      }
    }
  }
}
