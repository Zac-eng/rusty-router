use std::io::{Error, ErrorKind};
use std::str::FromStr;
// use std::io::{self, Error, ErrorKind};
use std::{env, io};
use std::net::Ipv4Addr;
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::Packet;
use pnet::ipnetwork::IpNetwork;
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};
use pnet_datalink::Channel::Ethernet;

pub struct LanInput {
  pub _ipv4_addr: Ipv4Addr,
  pub rx: Box<dyn DataLinkReceiver>,
}

pub struct LanOutput {
  dst_mac: MacAddr,
  self_mac: MacAddr,
  tx: Box<dyn DataLinkSender>,
}

impl LanInput {
  // pub fn receive(&mut self) -> io::Result<MutableEthernetPacket<'static>> {
  //   let packet = self.rx.next()?;
  //   match MutableEthernetPacket::owned(packet.to_vec()) {
  //     Some(etherframe) => Ok(etherframe),
  //     None => Err(Error::new(ErrorKind::InvalidData, "non_ethernet packet"))
  //   }
  // }

//   // pub fn classify(&self, etherframe: &EthernetPacket) -> Option<MutableIpv4Packet<'static>> {
//   //   match etherframe.get_ethertype() {
//   //     EtherTypes::Ipv4 => {
//   //       let ip_packet = MutableIpv4Packet::new(etherframe.payload())?; {
//   //         Some(packet) => packet,
//   //         None => return None
//   //       };
//   //       // if ip_packet.get_destination() == self.ipv4_addr {None}
//   //       // else {Some(ip_packet)}
//   //       Some(ip_packet)
//   //     }
//   //     _ => None
//   //   }
//   // }
}

impl LanOutput {
  pub fn ether_encap(&self, ip_packet: MutableIpv4Packet) -> Option<Vec<u8>> {
    let ip_packet_len = ip_packet.packet().len();
    let mut ether_buf = vec![0u8;ip_packet_len+14];
    let mut ether_frame = MutableEthernetPacket::new(&mut ether_buf)?;
    ether_frame.set_destination(self.dst_mac);
    ether_frame.set_source(self.self_mac);
    ether_frame.set_ethertype(EtherTypes::Ipv4);
    ether_frame.set_payload(ip_packet.packet());
    return Some(ether_buf)
  }
  pub fn send(&mut self, ether_frame: Vec<u8>) -> io::Result<()> {
    // return Ok also for the case no response from API
    match self.tx.send_to(&ether_frame, None) {
      Some(result) => return result,
      None => return Ok(())
    }
  }
}

pub fn construct_lan_interface(intf_env_var: &str, dst_env_var: &str) -> io::Result<(LanOutput, LanInput)> {
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
  let dst_mac = MacAddr::from_str(&env::var(dst_env_var).unwrap()).expect("invalid MAC address in env vars");
  let _ipv4_addr = get_ipv4_addr(&interface).expect("ipv4 address does not allocated");
  let output = LanOutput {dst_mac, self_mac: mac_addr, tx};
  let input = LanInput {_ipv4_addr, rx};
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
