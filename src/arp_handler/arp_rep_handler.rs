// use pnet::packet::arp::ArpPacket;

// use super::ArpHandler;

// impl ArpHandler {
  // gateway from outside of the struct
  // only arp_req_handle and arp_res_handle should be called from outside
  // pub fn handle_arp_reply(&mut self, arp_packet: ArpPacket) {
  //   let key_ip = arp_packet.get_sender_proto_addr();
  //   let value_mac = arp_packet.get_sender_hw_addr();
  //   match self.arp_table.get_mut(&arp_packet.get_sender_proto_addr()) {
  //     Some(current_mac) => {
  //       if *current_mac == value_mac {
  //         return ;
  //       }
  //       *current_mac = value_mac;
  //     },
  //     None => {
  //       self.arp_table.insert(key_ip, value_mac);
  //     }
  //   };
    // if there are some options to the first hop destination, here you should iter the packet queue
    // self.ip_que.into_iter().map(|ip_packet| {
    // })
  // }
// }
