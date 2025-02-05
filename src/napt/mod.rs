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

pub fn calc_translated_ip_checksum(original_checksum: u16, original_ip: &Ipv4Addr, new_ip: &Ipv4Addr) -> u16 {
  // let ip_header = &ip_packet.packet()[..ip_packet.get_header_length() as usize * 4];
  let original_octets = original_ip.octets();
  let new_octets = new_ip.octets();

  let original_ip_words = [u16::from_be_bytes([original_octets[0], original_octets[1]]), u16::from_be_bytes([original_octets[2], original_octets[3]])];
  let new_ip_words = [u16::from_be_bytes([new_octets[0], new_octets[1]]), u16::from_be_bytes([new_octets[2], new_octets[3]])];
  let mut new_sum = original_checksum as u32 + !original_ip_words[0] as u32 + !original_ip_words[1] as u32 + 2 + new_ip_words[0] as u32 + new_ip_words[1] as u32;
  while (new_sum >> 16) != 0 {
    new_sum = (new_sum & 0xFFFF) + (new_sum >> 16);
  }
  return new_sum as u16
}

pub fn calc_translated_transport_checksum(
  original_checksum: u16,
  original_ip: &Ipv4Addr,
  new_ip: &Ipv4Addr,
  original_port: u16,
  new_port: u16
) -> u16 {
  // let ip_header = &ip_packet.packet()[..ip_packet.get_header_length() as usize * 4];
  let original_octets = original_ip.octets();
  let new_octets = new_ip.octets();

  let original_ip_words = [u16::from_be_bytes([original_octets[0], original_octets[1]]), u16::from_be_bytes([original_octets[2], original_octets[3]])];
  let new_ip_words = [u16::from_be_bytes([new_octets[0], new_octets[1]]), u16::from_be_bytes([new_octets[2], new_octets[3]])];
  let mut new_sum = original_checksum as u32 + !original_ip_words[0] as u32 + !original_ip_words[1] as u32 + !original_port as u32 + 3 + new_ip_words[0] as u32 + new_ip_words[1] as u32 + new_port as u32;
  while (new_sum >> 16) != 0 {
    new_sum = (new_sum & 0xFFFF) + (new_sum >> 16);
  }
  return new_sum as u16
}
