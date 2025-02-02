use std::{collections::HashMap, net::Ipv4Addr};

use pnet::packet::{ip::IpNextHeaderProtocols::{Icmp, Tcp, Udp}, ipv4::Ipv4Packet};

mod tcp;

pub struct NAPTer {
  self_if: Ipv4Addr,
  tcp_map: HashMap<u16, (Ipv4Addr, u16)>,
  udp_map: HashMap<u16, (Ipv4Addr, u16)>,
  icmp_map: HashMap<u32, Ipv4Addr>
}

impl NAPTer {
  //translate packets coming from WAN
  pub fn translate_incoming<'p>(&self, packet: Ipv4Packet<'p>) -> Option<Ipv4Packet<'p>> {
    match packet.get_next_level_protocol() {
      Icmp => {},
      // Tcp => return self.translate_incoming_tcp(packet),
      Udp => {},
      _ => return None,
    }
    return None
  }
  //translate packets going to WAN
  pub fn translate_outgoing(&mut self, packet: Ipv4Packet) -> Option<Ipv4Packet<'static>> {
    match packet.get_next_level_protocol() {
      Icmp => {},
      Tcp => {},
      Udp => {},
      _ => return None
    }
    None
  }
}
