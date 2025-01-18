use std::{env, io, net::Ipv4Addr};
use pnet::packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use super::interface::Interface;

pub fn encapsulate_cipher(out_intf: &Interface, cipher: Vec<u8>) -> io::Result<MutableIpv4Packet> {
  let mut cipher_len = cipher.len();
  let mut ip_packet_buf = vec![0u8;cipher_len+20];
  let mut ip_packet = match MutableIpv4Packet::new(&mut ip_packet_buf) {
    Some(ipv4_packet) => ipv4_packet,
    None => return Err(io::Error::new(io::ErrorKind::OutOfMemory, "ip packet"))
  };
  ip_packet.set_version(4);
  ip_packet.set_total_length((cipher_len+20) as u16);
  ip_packet.set_identification(0);
  ip_packet.set_flags(0);
  ip_packet.set_fragment_offset(val);
  ip_packet.set_ttl(64);
  ip_packet.set_source(out_intf.ipv4addr);
  ip_packet.set_destination(env::var("TARGET_WAN_IP0").unwrap().parse::<Ipv4Addr>().unwrap());
  ip_packet.set_checksum(calc_ip_checksum(&ip_packet_buf[0..20]));
  ip_packet.set_payload(&cipher);
}

pub fn decapsulate_cipher(packet: Ipv4Packet) {}

fn calc_ip_checksum(ip_header: &[u8]) -> u16 {
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
