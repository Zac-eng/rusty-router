use std::{collections::HashMap, net::Ipv4Addr, sync::Mutex};

use pnet_datalink::Channel::Ethernet;
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};

pub struct Interface {
  pub interface: NetworkInterface,
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
    Ok(Self {
      interface,
      tx: Mutex::from(tx),
      rx,
      arp_table: HashMap::new()
    })
  }
}
