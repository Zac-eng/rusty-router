use std::{collections::HashMap, net::Ipv4Addr, sync::Mutex};

use pnet::ipnetwork::IpNetwork;
use pnet::packet::ipv4;
use pnet_datalink::Channel::Ethernet;
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};

pub struct WanSubIntf {
  pub interface: NetworkInterface,
  pub ipv4addr: Ipv4Addr,
  pub tx: Mutex<Box<dyn DataLinkSender>>,
  pub rx: Box<dyn DataLinkReceiver>,
  pub arp_table: HashMap<Ipv4Addr, MacAddr>
}
