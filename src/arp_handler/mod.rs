use std::net::Ipv4Addr;
use pnet::packet::arp::{self, ArpOperation, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::Packet;
use pnet_datalink::{DataLinkSender, MacAddr};

// mod arp_rep_handler;
mod arp_req_handler;
mod ip_packet_sender;

pub struct ArpHandler {
  ip_addr: Ipv4Addr,
  mac_addr: MacAddr,
  tx: Box<dyn DataLinkSender>,
  // ip_que: VecDeque<Ipv4Packet<'static>>,
  // arp_table: HashMap<Ipv4Addr, MacAddr>
}

impl ArpHandler {
  pub fn handle_arp(&mut self, frame: EthernetPacket) {
    if let Some(arp_packet) = ArpPacket::new(&frame.payload()) {
      match arp_packet.get_operation() {
        arp::ArpOperations::Request => self.handle_arp_request(arp_packet),
        arp::ArpOperations::Reply => return ,
        _ => return
      }
    }
  }

  // async fn broadcast(&self, dst_addr: &Ipv4Addr) -> Result<MacAddr, &'static str> {
  //   let interfaces = pnet_datalink::interfaces();
  //   let mut etherframe_buf = [0u8;42];
  //   let mut ether_frame = MutableEthernetPacket::new(&mut etherframe_buf).expect("failed to create ether frame");
  //   let mut arp_buf = [0u8;28];
  //   let mut arp_packet = MutableArpPacket::new(&mut arp_buf).expect("failed to create arp packet");
  //   arp_packet.set_hardware_type(arp::ArpHardwareTypes::Ethernet);
  //   arp_packet.set_protocol_type(EtherTypes::Ipv4);
  //   arp_packet.set_hw_addr_len(6);
  //   arp_packet.set_proto_addr_len(4);
  //   arp_packet.set_operation(arp::ArpOperations::Request);
  //   arp_packet.set_sender_proto_addr(Ipv4Addr::new(192, 168, 1, 100));
  //   arp_packet.set_target_hw_addr(MacAddr::zero());
  //   arp_packet.set_target_proto_addr(Ipv4Addr::new(192, 168, 1, 1));

  //   // Copy ARP packet into Ethernet payload
  //   ether_frame.set_payload(arp_packet.packet());
  //   ether_frame.set_destination(pnet_datalink::MacAddr::broadcast());
  //   ether_frame.set_ethertype(pnet::packet::ethernet::EtherTypes::Arp);
  //   for interface in interfaces {
  //     match pnet_datalink::channel(&interface, Default::default()) {
  //       Ok(Ethernet(mut tx, _)) =>  {
  //         arp_packet.set_sender_hw_addr(source_mac);
  //         tx.send_to(ether_frame, None);
  //       },
  //       _ => {}
  //     }
  //   }
  //   thread::sleep(Duration::from_secs(2));
  //   match self.arp_table.lock().unwrap().get(dst_addr) {
  //     Some(mac_addr) => return Ok(mac_addr.clone()),
  //     None => return Err("IP address not found"),
  //   }
  // }
}

fn form_arp_packet(
  src_mac: MacAddr,
  src_ip: Ipv4Addr,
  tgt_mac: MacAddr,
  tgt_ip: Ipv4Addr,
  operation: ArpOperation
 ) -> ArpPacket<'static> {
  let arp_buf = vec![0u8;28];
  let mut arp_packet = MutableArpPacket::owned(arp_buf).unwrap();
  arp_packet.set_hardware_type(arp::ArpHardwareTypes::Ethernet);
  arp_packet.set_protocol_type(pnet::packet::ethernet::EtherTypes::Ipv4);
  arp_packet.set_hw_addr_len(6);
  arp_packet.set_proto_addr_len(4);
  arp_packet.set_operation(operation);
  arp_packet.set_sender_hw_addr(src_mac);
  arp_packet.set_sender_proto_addr(src_ip);
  arp_packet.set_target_hw_addr(tgt_mac);
  arp_packet.set_target_proto_addr(tgt_ip);
  return ArpPacket::owned(Vec::from(arp_packet.packet())).unwrap()
}

fn etherframe_from_arp(arp_packet: ArpPacket) -> EthernetPacket<'static> {
  let dst_mac = arp_packet.get_target_hw_addr();
  let src_mac = arp_packet.get_sender_hw_addr();
  let frame_buf = vec![0u8;42];
  let mut etherframe = MutableEthernetPacket::owned(frame_buf).unwrap();
  etherframe.set_destination(dst_mac);
  etherframe.set_source(src_mac);
  return EthernetPacket::owned(Vec::from(etherframe.packet())).unwrap()
}
