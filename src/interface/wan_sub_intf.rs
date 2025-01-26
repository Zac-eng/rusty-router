use std::{collections::HashMap, net::Ipv4Addr, sync::Mutex};
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};

pub struct WanSubInput {
  pub interface: NetworkInterface,
  pub ipv4addr: Ipv4Addr,
  pub tx: Mutex<Box<dyn DataLinkSender>>,
  pub rx: Box<dyn DataLinkReceiver>,
  pub arp_table: HashMap<Ipv4Addr, MacAddr>
}

pub struct WanSubOutput {
  pub self_ip: Ipv4Addr,
  pub dest_ip: Ipv4Addr,
  pub self_mac: MacAddr,
  pub first_hop_mac: MacAddr,
  pub tx: Box<dyn DataLinkSender>
}
