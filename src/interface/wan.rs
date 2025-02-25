use std::env;
use std::io::{self, Error, ErrorKind};
use std::net::Ipv4Addr;
use std::str::FromStr;
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::MutablePacket;
use pnet::ipnetwork::IpNetwork;
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};
use pnet_datalink::Channel::Ethernet;

use crate::napt::calc_ip_checksum;

pub struct WanInput {
  pub ipv4_addr: Ipv4Addr,
  pub rx: Box<dyn DataLinkReceiver>,
}

pub struct WanOutput {
  self_ip: Ipv4Addr,
  dst_ip: Ipv4Addr,
  dst_mac: MacAddr,
  self_mac: MacAddr,
  tx: Box<dyn DataLinkSender>,
}

impl WanInput {
  pub fn receive(&mut self) -> io::Result<MutableEthernetPacket<'static>> {
    let packet = self.rx.next()?;
    match MutableEthernetPacket::owned(packet.to_vec()) {
      Some(etherframe) => Ok(etherframe),
      None => Err(Error::new(ErrorKind::InvalidData, "non_ethernet packet"))
    }
  }

  pub fn classify<'a>(&self, etherframe: &'a mut MutableEthernetPacket<'a>) -> Option<MutableIpv4Packet<'a>> {
    match etherframe.get_ethertype() {
      EtherTypes::Ipv4 => {
        if let Some(ip_packet) = MutableIpv4Packet::new(etherframe.payload_mut()) {
          if ip_packet.get_destination() == self.ipv4_addr {
            return Some(ip_packet)
          }
        };
        return None
      }
      _ => None
    }
  }
}

impl WanOutput {
  pub fn ip_encap(&self, ip_packet: &mut MutableIpv4Packet) {
    ip_packet.set_source(self.self_ip);
    ip_packet.set_destination(self.dst_ip);
    ip_packet.set_checksum(0);
    ip_packet.set_checksum(calc_ip_checksum(ip_packet));
  }
  pub fn ether_encap(&self, ether_frame: &mut MutableEthernetPacket) {
    ether_frame.set_destination(self.dst_mac);
    ether_frame.set_source(self.self_mac);
  }
  pub fn send(&mut self, ether_frame: &[u8]) -> io::Result<()> {
    // return Ok also for the case no response from API
    match self.tx.send_to(ether_frame, None) {
      Some(result) => return result,
      None => return Ok(())
    }
  }
}

pub fn construct_wan_interface(intf_env_var: &str, dst_ip_env: &str, dst_mac_env: &str) -> io::Result<(WanOutput, WanInput)> {
  let interface_name = env::var(intf_env_var).unwrap();
  let interface = match pnet_datalink::interfaces().into_iter().find(|intf| intf.name == interface_name) {
    Some(interface) => interface,
    None => return Err(Error::new(ErrorKind::NotFound, format!("interface: {:?}", interface_name)))
  };
  let (tx, rx) = match pnet_datalink::channel(&interface, Default::default()) {
    Ok(Ethernet(tx, rx)) => (tx, rx),
    Ok(_) => return Err(Error::new(ErrorKind::InvalidData, "non-ethernet interface")),
    Err(e) => return Err(e)
  };
  let mac_addr = interface.mac.expect("no mac address allocated");
  let dst_ip = Ipv4Addr::from_str(&env::var(dst_ip_env).unwrap()).expect("invalid IPv4 address in env vars");
  let dst_mac = MacAddr::from_str(&env::var(dst_mac_env).unwrap()).expect("invalid MAC address in env vars");
  let ipv4_addr = get_ipv4_addr(&interface).expect("ipv4 address does not allocated");
  let output = WanOutput {self_ip: ipv4_addr, dst_ip, dst_mac, self_mac: mac_addr, tx};
  let input = WanInput {ipv4_addr, rx};
  return Ok((output, input))
}

fn get_ipv4_addr(interface: &NetworkInterface) -> Option<Ipv4Addr> {
  interface.ips.iter().find_map(|ipaddr| {
    if let IpNetwork::V4(ipv4addr) = ipaddr {
      return Some(ipv4addr.ip());
    }
    else {
      return None;
    }
  })
}
