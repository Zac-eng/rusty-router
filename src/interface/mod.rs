use std::{collections::HashMap, net::Ipv4Addr, sync::Mutex};

use pnet::ipnetwork::IpNetwork;
use pnet::packet::ipv4;
use pnet_datalink::Channel::Ethernet;
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};

pub struct Interface {
  pub interface: NetworkInterface,
  pub ipv4addr: Ipv4Addr,
  pub tx: Mutex<Box<dyn DataLinkSender>>,
  pub rx: Box<dyn DataLinkReceiver>,
  pub arp_table: HashMap<Ipv4Addr, MacAddr>
}

impl Interface {
  // consume passed interface, but includes it as attribute
  pub fn new(mut interface: NetworkInterface) -> Result<Self, &'static str> {
    let (mut tx, rx) = match pnet_datalink::channel(&mut interface, Default::default()) {
      Ok(Ethernet(tx, rx)) => (tx, rx),
      _ => return Err("ethernet interface not found"),
    };
    let ipv4addr = match get_ipv4_addr(&interface) {
      Some(ipv4) => ipv4,
      None => return Err("interface does not have ipv4 address")
    };
    Ok(Self {
      interface,
      ipv4addr,
      tx: Mutex::from(tx),
      rx,
      arp_table: HashMap::new()
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
