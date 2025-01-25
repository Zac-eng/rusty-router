use std::io::{self, Error, ErrorKind};
use std::net::IpAddr;
use std::{collections::HashMap, net::Ipv4Addr, sync::Mutex};

use pnet::ipnetwork::IpNetwork;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::{self, Ipv4Packet};
use pnet_datalink::Channel::Ethernet;
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};

use super::Interface;


pub struct Lan {
  pub intf: Interface,
  //arph: ArpHandler,
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
  pub fn run(&mut self) -> io::Result<()> {
    loop {
      let packet = self.intf.rx.next()?;
      let ether_frame = match EthernetPacket::new(packet) {
        Some(frame) => frame,
        None => continue
      };
      //if you want to drop broadcast, implement it here
      match ether_frame.get_ethertype() {
        EtherTypes::Arp => {},
        // EtherTypes::Arp => self.arph.handle_arp(),
        EtherTypes::Ipv4 => {},
        _ => continue
      }
    }
  }
}
