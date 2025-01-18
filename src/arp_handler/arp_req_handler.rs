use std::io;
use std::net::Ipv4Addr;

use pnet::packet::{arp::{ArpPacket, MutableArpPacket}, ethernet::MutableEthernetPacket};
use pnet_datalink::{DataLinkSender, MacAddr};

use super::ARPHandler;

impl ARPHandler {
  pub fn arp_req_handle(&self, arp_packet: &ArpPacket) -> io::Result<()> {
    let mut ether_frame_buf = [0u8;42];
    let mut arp_packet_buf = [0u8;28];
    let mut ether_frame = MutableEthernetPacket::new(&mut ether_frame_buf).unwrap();
    let mut arp_packet = MutableArpPacket::new(&mut arp_packet_buf).unwrap();
    // match self.look_up_table(arp_packet.get_target_proto_addr()) {
    //   Some(mac_addr) => arp_packet.set_
    // }
    return Ok(())
  }

  fn look_up_table(&self, target_addr: &Ipv4Addr) -> Option<MacAddr> {
    let arp_table_locked = self.arp_table.lock().unwrap();
    match arp_table_locked.get(target_addr) {
      Some(mac_addr) => Some(mac_addr.clone()),
      None => None
    }
  }
  // pub fn respond(&self, dst_channel: &Box<dyn DataLinkSender>, target_addr: &Ipv4Addr) {
  //   let mac_addr = self.get_mac_addr(target_addr);
  // }
}