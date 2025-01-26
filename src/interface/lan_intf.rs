use std::io::{self, Error, ErrorKind};
use std::sync::Arc;
use pnet_datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::{self, Ipv4Packet};
use pnet::packet::Packet;
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};

use crate::arp_handler::ArpHandler;
use crate::{crypto, capsule, arp_handler};
use super::Interface;

pub struct LanInput {
  rx: Box<dyn DataLinkReceiver>,
}

pub struct LanOutput {
  mac: MacAddr,
  tx: Box<dyn DataLinkSender>,
}

// impl Interface for LanIntf {}

// impl Lan {
//   pub fn new(
//     intf_name: &str,
//     rx: Box<dyn DataLinkReceiver>,
//     lan_tx: Arc<ArpHandler>,
//     wan0_tx: Arc<ArpHandler>,
//     wan1_tx: Arc<ArpHandler>
//   ) -> io::Result<Self> {
//     let intf = match pnet_datalink::interfaces().into_iter().find(|i| i.name == intf_name) {
//       Some(intf) => intf,
//       None => return Err(Error::new(ErrorKind::NotFound, "interface"))
//     };
//     // let (_, rx) = match pnet_datalink::channel(&intf, Default::default()) {
//     //   Ok(Ethernet(_, rx)) => (_, rx),
//     //   Ok(some_intf) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "channel")),
//     //   Err(e) => return Err(e),
//     // };
//     let intf = Interface::new(intf)?;
//     return Ok(Self {intf, rx, lan_tx, wan0_tx, wan1_tx})
//   }
//   pub fn run(&mut self) {
//     loop {
//       let packet = self.intf.rx.next().unwrap();
//       let ether_frame = match EthernetPacket::new(packet) {
//         Some(frame) => frame,
//         None => continue
//       };
//       //if you want to drop broadcast, implement it here
//       match ether_frame.get_ethertype() {
//         EtherTypes::Arp => {},
//         // EtherTypes::Arp => self.arph.handle_arp(),
//         EtherTypes::Ipv4 => handle_ip_packet(ether_frame),
//         _ => continue
//       }
//     }
//   }
// }

// fn handle_ip_packet(frame: EthernetPacket) {
//   if let Ok((key, iv)) = crypto::load_crypto_info() {
//     if let Ok(encrypted) = crypto::encrypt_packet(&frame.packet(), &key, &iv) {
//       // capsule::encapsulate_cipher(, encrypted);
//     };
//   }
// }
