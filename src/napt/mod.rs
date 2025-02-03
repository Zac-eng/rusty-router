use std::{collections::HashMap, hash::Hash, net::Ipv4Addr};

use pnet::packet::{ip::IpNextHeaderProtocols::{Icmp, Tcp, Udp}, ipv4::{Ipv4Packet, MutableIpv4Packet}};

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
  pub fn translate_incoming<'p>(&self, packet: MutableIpv4Packet<'p>) -> Option<MutableIpv4Packet<'p>> {
    match packet.get_next_level_protocol() {
      // Icmp => {},
      Tcp => self.translate_incoming_tcp(packet),
      // Udp => {},
      _ => None,
    }
  }
  //translate packets going to WAN
  pub fn translate_outgoing(&mut self, packet: MutableIpv4Packet) -> Option<MutableIpv4Packet<'static>> {
    match packet.get_next_level_protocol() {
      // Icmp => {},
      Tcp => self.translate_outgoing_tcp(packet),
      // Udp => {},
      _ => None
    }
  }
}
