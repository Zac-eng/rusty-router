use std::io::{self, Error, ErrorKind};
use std::env;
use std::str::FromStr;
use pnet_datalink::MacAddr;
use pnet_datalink::Channel::Ethernet;

use super::{get_ipv4_addr, IntfInput, IntfOutput};

pub fn construct_interface(interface_name: &str, dst_env_var: &str) -> io::Result<(IntfOutput, IntfInput)> {
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
  let ipv4_addr = get_ipv4_addr(&interface).expect("ipv4 address does not allocated");
  let output = IntfOutput {dst_mac, self_mac: mac_addr, tx};
  let input = IntfInput {ipv4_addr, rx};
  return Ok((output, input))
}
