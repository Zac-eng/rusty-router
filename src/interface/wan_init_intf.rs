use std::collections::VecDeque;
use std::sync::Arc;
use std::{collections::HashMap, net::Ipv4Addr, sync::Mutex};

use pnet::ipnetwork::IpNetwork;
use pnet::packet::ipv4::{self, Ipv4Packet};
use pnet_datalink::Channel::Ethernet;
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};

pub struct WanInitInput {
  pub ip_que: Arc<VecDeque<Ipv4Packet<'static>>>,
  pub rx: Box<dyn DataLinkReceiver>,
}

pub struct WanInitOutput {
  pub self_ip: Ipv4Addr,
  pub dest_ip: Ipv4Addr,
  pub self_mac: MacAddr,
  pub first_hop_mac: MacAddr,
  pub tx: Box<dyn DataLinkSender>
}
