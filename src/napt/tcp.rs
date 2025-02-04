use std::ops::Index;

use pnet::packet::{ipv4::MutableIpv4Packet, tcp:: MutableTcpPacket, Packet};

use super::{calc_ip_checksum, NAPTer};

impl NAPTer {
  pub fn translate_incoming_tcp(&self, ip_packet: MutableIpv4Packet) -> Option<MutableIpv4Packet<'static>> {
    let mut new_ip_packet = MutableIpv4Packet::owned(ip_packet.packet().to_vec()).unwrap();
    let mut new_tcp_packet = MutableTcpPacket::owned(ip_packet.payload().to_vec()).unwrap();
    if let Some(local_dest) = self.tcp_map.get(&u16::from_be(new_tcp_packet.get_destination())) {
      new_tcp_packet.set_destination(local_dest.1.to_be());
      new_ip_packet.set_destination(local_dest.0);
      new_ip_packet.set_payload(new_tcp_packet.packet());
    }
    return Some(new_ip_packet)
  }

  pub fn translate_outgoing_tcp(&mut self, ip_packet: MutableIpv4Packet) -> Option<MutableIpv4Packet<'static>> {
    let mut new_ip_packet = MutableIpv4Packet::owned(ip_packet.packet().to_vec()).unwrap();
    let mut new_tcp_packet = MutableTcpPacket::owned(ip_packet.payload().to_vec()).unwrap();
    let original_ip = new_ip_packet.get_source();
    let original_port = u16::from_be(new_tcp_packet.get_source());
    let mut translated_port = original_port;
    while let Some(local_dst) = self.tcp_map.get(&translated_port) {
      if *local_dst == (original_ip, original_port) {break;}
      translated_port+=1;
    }
    self.tcp_map.insert(translated_port, (new_ip_packet.get_source(), original_port));
    new_ip_packet.set_source(self.self_ip);
    new_ip_packet.set_checksum(calc_ip_checksum(&new_ip_packet));
    new_tcp_packet.set_source(translated_port.to_be());
    new_ip_packet.set_payload(new_tcp_packet.packet());
    return Some(new_ip_packet)
  }
}
