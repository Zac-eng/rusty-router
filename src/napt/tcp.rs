use std::{io, net::Ipv4Addr};
use std::cell::RefCell;

use pnet::packet::tcp::{self, TcpPacket};
use pnet::packet::{ipv4::{Ipv4Packet, MutableIpv4Packet}, tcp:: MutableTcpPacket, Packet};
use pnet::packet::MutablePacket;

use super::NAPTer;

impl NAPTer {
  pub fn translate_incoming_tcp<'p>(&self, mut ip_packet: MutableIpv4Packet<'p>) -> Option<MutableIpv4Packet<'p>> {
    // let buffer_ref = RefCell::new(ip_packet.packet_mut());
    // let mut external_port = 0;
    // let mut tcp_buf = buffer_ref.borrow_mut();
    // let mut local_dst: &(Ipv4Addr, u16);
    // if let Some(tcp_packet) = MutableTcpPacket::new(&mut tcp_buf) {
    //   if let Some(local_dst) = self.tcp_map.get(&tcp_packet.get_destination()) {
    //     tcp_packet.set_destination(local_dst.1);
    //   } else {return None}
    // } else {return None}
    // drop(tcp_buf);
    // let mut ip_buf = buffer_ref.borrow_mut();
    // let mut ip_packet = MutableIpv4Packet::new(&mut ip_buf)?;
    // ip_packet.set_destination(local_dst.0);
    // return Some(ip_packet)
    // let tcp_buf = ip_packet.payload();
    // let external_port: u16;
    // if let Some(tcp_packet) = TcpPacket::new(tcp_buf) {
    //   external_port = u16::from_be(tcp_packet.get_destination());
    // } else {return None}
    // {
    //   ip_packet
    // }
    None
  }
}

  // pub fn translate_outgoing_tcp(&mut self, ip_packet: Ipv4Packet) -> Option<Ipv4Packet<'static>> {
  //   if ()
  // }