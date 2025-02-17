use std::{collections::HashMap, net::Ipv4Addr};

use pnet::packet::{ip::IpNextHeaderProtocols::{Icmp, Tcp, Udp}, ipv4::{Ipv4Packet, MutableIpv4Packet}, tcp::MutableTcpPacket, Packet, MutablePacket};

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
  compute_checksum(&ip_packet.packet()[..20])
}

pub fn calc_tcp_checksum(ip_packet: &MutableIpv4Packet) -> u16 {
  let mut tcp = MutableTcpPacket::owned(ip_packet.payload().to_vec()).unwrap();
  tcp.set_checksum(0);
  let mut tcp_checksum_buf: Vec<u8> = Vec::new();
  tcp_checksum_buf.append(&mut ip_packet.packet()[12..20].to_vec());
  let tcp_packet_len = tcp.packet().len();
  tcp_checksum_buf.append(&mut vec![0u8, 6u8, (tcp_packet_len/256) as u8, (tcp_packet_len%256)as u8]);
  tcp_checksum_buf.append(&mut tcp.packet().to_vec());
  compute_checksum(&tcp_checksum_buf)
}

// pub fn calc_translated_ip_checksum(original_checksum: u16, original_ip: &Ipv4Addr, new_ip: &Ipv4Addr) -> u16 {
//   // let ip_header = &ip_packet.packet()[..ip_packet.get_header_length() as usize * 4];
//   let original_octets = original_ip.octets();
//   let new_octets = new_ip.octets();

//   let original_ip_words = [u16::from_be_bytes([original_octets[0], original_octets[1]]), u16::from_be_bytes([original_octets[2], original_octets[3]])];
//   let new_ip_words = [u16::from_be_bytes([new_octets[0], new_octets[1]]), u16::from_be_bytes([new_octets[2], new_octets[3]])];
//   let mut new_sum = original_checksum as u32 + !original_ip_words[0] as u32 + !original_ip_words[1] as u32 + 2 + new_ip_words[0] as u32 + new_ip_words[1] as u32;
//   while (new_sum >> 16) != 0 {
//     new_sum = (new_sum & 0xFFFF) + (new_sum >> 16);
//   }
//   return new_sum as u16
// }

// pub fn calc_translated_transport_checksum(
//   original_checksum: u16,
//   original_ip: &Ipv4Addr,
//   new_ip: &Ipv4Addr,
//   original_port: u16,
//   new_port: u16
// ) -> u16 {
//   // let ip_header = &ip_packet.packet()[..ip_packet.get_header_length() as usize * 4];
//   let original_octets = original_ip.octets();
//   let new_octets = new_ip.octets();

//   let original_ip_words = [u16::from_be_bytes([original_octets[0], original_octets[1]]), u16::from_be_bytes([original_octets[2], original_octets[3]])];
//   let new_ip_words = [u16::from_be_bytes([new_octets[0], new_octets[1]]), u16::from_be_bytes([new_octets[2], new_octets[3]])];
//   let mut new_sum = original_checksum as u32 + !original_ip_words[0] as u32 + !original_ip_words[1] as u32 + !original_port as u32 + 3 + new_ip_words[0] as u32 + new_ip_words[1] as u32 + new_port as u32;
//   while (new_sum >> 16) != 0 {
//     new_sum = (new_sum & 0xFFFF) + (new_sum >> 16);
//   }
//   return new_sum as u16
// }

fn compute_checksum(mut data: &[u8]) -> u16 {
  let mut sum: u32 = 0;

  while data.len() >= 2 {
      let word = ((data[0] as u16) << 8) | (data[1] as u16);
      sum += word as u32;
      data = &data[2..];
  }
  if !data.is_empty() {
      sum += (data[0] as u32) << 8;
  }
  while (sum >> 16) != 0 {
      sum = (sum & 0xFFFF) + (sum >> 16);
  }

  !(sum as u16)
}
