pub mod lan_intf;
pub mod wan_init_intf;
pub mod wan_sub_intf;

use std::io::{self, Error, ErrorKind};
use std::{collections::HashMap, net::Ipv4Addr, sync::Mutex};

use pnet::ipnetwork::IpNetwork;
use pnet::packet::arp::MutableArpPacket;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::Packet;
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};
use pnet_datalink::Channel::Ethernet;

// pub trait Interface {
//   fn set_interface(intf: NetworkInterface);
//   fn lock_own_sender(&self) -> impl DataLinkSender;
//   fn get_ipv4_addr(&self) -> &Ipv4Addr;
//   // fn respond_arp(&self, target: &Ipv4Addr) {
//   //   if *target == *self.get_ipv4_addr() {
//   //     let mut tx = self.lock_own_sender();
//   //     let mut arp_buf = [0u8;28];
//   //     let mut ether_buf = [0u8;42];
//   //     let mut arp_packet = MutableArpPacket::new(&mut arp_buf).unwrap();
//   //     let mut ether_frame = MutableEthernetPacket::new(&mut ether_buf).unwrap();
//   //     // arp_packet.set_hardware_type(val);
//   //     // ether_frame.set_payload(&arp_packet.packet());
//   //     tx.send_to(&ether_frame.packet(), None);
//   //   }
//   // }
// }

pub struct Interface {
  pub interface: NetworkInterface,
  pub ipv4addr: Ipv4Addr,
  pub tx: Mutex<Box<dyn DataLinkSender>>,
  pub rx: Box<dyn DataLinkReceiver>,
}

impl Interface {
  // consume passed interface, but includes it as attribute
  pub fn new(mut interface: NetworkInterface) -> io::Result<Self> {
    let (tx, rx) = match pnet_datalink::channel(&mut interface, Default::default()) {
      Ok(Ethernet(tx, rx)) => (tx, rx),
      _ => return Err(Error::new(ErrorKind::NotFound, "channel")),
    };
    let ipv4addr = match get_ipv4_addr(&interface) {
      Some(ipv4) => ipv4,
      None => return Err(Error::new(ErrorKind::NotFound, "ipv4"))
    };
    Ok(Self {
      interface,
      ipv4addr,
      tx: Mutex::from(tx),
      rx
    })
  }
}

pub fn get_ipv4_addr(interface: &NetworkInterface) -> Option<Ipv4Addr> {
  interface.ips.iter().find_map(|ipaddr| {
    if let IpNetwork::V4(ipv4addr) = ipaddr {
      return Some(ipv4addr.ip());
    }
    else {
      return None;
    }
  })
}
