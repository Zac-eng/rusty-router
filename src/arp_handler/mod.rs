use std::thread;
use std::time::Duration;
use std::{collections::HashMap, net::Ipv4Addr, sync::Mutex};
use pnet::packet::arp::{self, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet_datalink::MacAddr;
use pnet_datalink::Channel::Ethernet;

mod arp_res_handler;
mod arp_req_handler;

struct ARPHandler {
  arp_table: Mutex<HashMap<Ipv4Addr, MacAddr>>
}

impl ARPHandler {
  // async fn get_mac_addr(&self, dst_addr: &Ipv4Addr) -> Result<MacAddr, &'static str> {
  //   match self.arp_table.lock().unwrap().get(dst_addr) {
  //     Some(mac_addr) => return Ok(mac_addr.clone()),
  //     None => return self.broadcast(dst_addr).await,
  //   }
  // }

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
