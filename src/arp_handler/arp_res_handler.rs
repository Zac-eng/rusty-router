use pnet::packet::arp::ArpPacket;

use super::ARPHandler;

impl ARPHandler {
  // gateway from outside of the struct
  // only arp_req_handle and arp_res_handle should be called from outside
  pub fn arp_res_handle(&self, arp_packet: &ArpPacket) {
    self.add_to_table(arp_packet);
  }

  // no return value: hash map only returns prior value for added key, 
  // but it is old ip-mac information here, so discard it 
  fn add_to_table(&self, arp_packet: &ArpPacket) {
    let mut arp_table_locked = self.arp_table.lock().unwrap();
    arp_table_locked.insert(arp_packet.get_sender_proto_addr(), arp_packet.get_sender_hw_addr());
  }
}
