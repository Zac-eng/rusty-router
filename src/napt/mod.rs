use std::{collections::HashMap, net::Ipv4Addr};

use pnet::packet::{ip::IpNextHeaderProtocols::{Icmp, Tcp, Udp}, ipv4::MutableIpv4Packet, Packet};

mod tcp;

pub struct NAPTer {
  self_ip: Ipv4Addr,
  tcp_map: HashMap<u16, (Ipv4Addr, u16)>,
  udp_map: HashMap<u16, (Ipv4Addr, u16)>,
  icmp_map: HashMap<u32, Ipv4Addr>
}

impl NAPTer {
  pub fn new(self_ip: Ipv4Addr) -> Self {
    Self {
      self_ip,
      tcp_map: HashMap::new(),
      udp_map: HashMap::new(),
      icmp_map: HashMap::new()
    }
  }
  //translate packets coming from WAN
  pub fn translate_incoming(&self, packet: MutableIpv4Packet<'static>) -> Option<MutableIpv4Packet<'static>> {
    match packet.get_next_level_protocol() {
      // Icmp => {},
      Tcp => self.translate_incoming_tcp(packet),
      // Udp => {},
      _ => Some(packet)
    }
  }
  //translate packets going to WAN
  pub fn translate_outgoing(&mut self, packet: MutableIpv4Packet<'static>) -> Option<MutableIpv4Packet<'static>> {
    match packet.get_next_level_protocol() {
      // Icmp => {},
      Tcp => self.translate_outgoing_tcp(packet),
      // Udp => {},
      _ => Some(packet)
    }
  }
}

pub fn calc_ip_checksum(ip_packet: &MutableIpv4Packet) -> u16 {
  let ip_header = &ip_packet.packet()[..ip_packet.get_header_length() as usize * 4];
  let mut sum = 0u32;
  for chunk in ip_header.chunks(2) {
    let word = if chunk.len() == 2 {
      u16::from_be_bytes([chunk[0], chunk[1]])
    } else {
      u16::from_be_bytes([chunk[0], 0])
    };
    sum += word as u32;
  }
  while (sum >> 16) != 0 {
    sum = (sum & 0xFFFF) + (sum >> 16);
  }
  !(sum as u16)
}
