use pnet::packet::{arp::{self, ArpPacket}, Packet};

use super::{etherframe_from_arp, form_arp_packet, ArpHandler};

impl ArpHandler {
  pub fn handle_arp_request(&mut self, arp_packet: ArpPacket) {
    if arp_packet.get_target_proto_addr() != self.ip_addr {return}
    let dst_ip = arp_packet.get_sender_proto_addr();
    let dst_mac = arp_packet.get_sender_hw_addr();
    let arp_packet = form_arp_packet(self.mac_addr, self.ip_addr, dst_mac, dst_ip, arp::ArpOperations::Reply);
    let ether_frame = etherframe_from_arp(arp_packet);
    self.tx.send_to(ether_frame.packet(), None);
  }
}
