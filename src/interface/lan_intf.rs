use std::io::{self, Error, ErrorKind};
use pnet::packet::ethernet::{EtherTypes, Ethernet, EthernetPacket};
use pnet::packet::ipv4::{self, Ipv4Packet};
use pnet::packet::Packet;
use pnet_datalink::{DataLinkSender, NetworkInterface};

use crate::{crypto, capsule, arp_handler};
use super::Interface;

pub struct Lan {
  pub intf: Interface,
  // arph: arp_handler::ArpHandler,
  // pub arp_table: HashMap<Ipv4Addr, MacAddr>
}

// impl Interface for LanIntf {}

impl Lan {
  pub fn new(intf_name: &str) -> io::Result<Self> {
    let intf = match pnet_datalink::interfaces().into_iter().find(|i| i.name == intf_name) {
      Some(intf) => intf,
      None => return Err(Error::new(ErrorKind::NotFound, "interface"))
    };
    let intf = Interface::new(intf)?;
    return Ok(Self {intf})
  }
  pub fn run(&mut self) {
    loop {
      let packet = self.intf.rx.next().unwrap();
      let ether_frame = match EthernetPacket::new(packet) {
        Some(frame) => frame,
        None => continue
      };
      //if you want to drop broadcast, implement it here
      match ether_frame.get_ethertype() {
        EtherTypes::Arp => {},
        // EtherTypes::Arp => self.arph.handle_arp(),
        EtherTypes::Ipv4 => handle_ip_packet(ether_frame),
        _ => continue
      }
    }
  }
}

fn handle_ip_packet(frame: EthernetPacket) {
  if let Ok((key, iv)) = crypto::load_crypto_info() {
    if let Ok(encrypted) = crypto::encrypt_packet(&frame.packet(), &key, &iv) {
      // capsule::encapsulate_cipher(, encrypted);
    };
  }
}
