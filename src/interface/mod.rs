use std::io::{self, Error, ErrorKind};
use std::net::Ipv4Addr;
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::Packet;
use pnet::{ipnetwork::IpNetwork, packet::ethernet::EthernetPacket};
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};

pub mod constructor;
pub mod thread_funcs;

pub struct IntfInput {
  pub ipv4_addr: Ipv4Addr,
  pub rx: Box<dyn DataLinkReceiver>,
}

pub struct IntfOutput {
  dst_mac: MacAddr,
  self_mac: MacAddr,
  tx: Box<dyn DataLinkSender>,
}

impl IntfInput {
  pub fn receive(&mut self) -> io::Result<MutableEthernetPacket<'static>> {
    let packet = self.rx.next()?;
    match MutableEthernetPacket::owned(packet.to_vec()) {
      Some(etherframe) => Ok(etherframe),
      None => Err(Error::new(ErrorKind::InvalidData, "non_ethernet packet"))
    }
  }

  // pub fn classify(&self, etherframe: EthernetPacket) -> Option<MutableIpv4Packet<'static>> {
  //   match etherframe.get_ethertype() {
  //     EtherTypes::Ipv4 => {
  //       let ip_packet = MutableIpv4Packet::owned(etherframe.payload().to_vec())?; {
  //         Some(packet) => packet,
  //         None => return None
  //       };
  //       // if ip_packet.get_destination() == self.ipv4_addr {None}
  //       // else {Some(ip_packet)}
  //       Some(ip_packet)
  //     }
  //     _ => None
  //   }
  // }
}

impl IntfOutput {
  pub fn ether_encap(&self, ip_packet: MutableIpv4Packet) -> Option<EthernetPacket<'static>> {
    let ip_packet_len = ip_packet.packet().len();
    let mut ether_frame = MutableEthernetPacket::owned(vec![0u8;ip_packet_len+14])?;
    ether_frame.set_destination(self.dst_mac);
    ether_frame.set_source(self.self_mac);
    ether_frame.set_ethertype(EtherTypes::Ipv4);
    ether_frame.set_payload(ip_packet.packet());
    let ether_frame = EthernetPacket::owned(ether_frame.packet().to_vec()).unwrap();
    return Some(ether_frame)
  }
  pub fn send(&mut self, ether_frame: EthernetPacket) -> io::Result<()> {
    // return Ok also for the case no response from API
    match self.tx.send_to(ether_frame.packet(), None) {
      Some(result) => return result,
      None => return Ok(())
    }
  }
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
