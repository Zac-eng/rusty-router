use std::{env, io};

use pnet_datalink::{DataLinkReceiver, DataLinkSender};
use pnet_datalink::Channel::Ethernet;

pub fn create_wan_channels(wan_envvars: Vec<&str>) -> io::Result<(Vec<Box<dyn DataLinkSender>>, Vec<Box<dyn DataLinkReceiver>>)> {
  let mut out_channels: Vec<Box<dyn DataLinkSender>> = Vec::new();
  let mut in_channels: Vec<Box<dyn DataLinkReceiver>> = Vec::new();

  for env_var in wan_envvars {
    let intf_name = env::var(env_var).unwrap();
    if let Some(interface) = pnet_datalink::interfaces().into_iter().find(|intf| intf.name == intf_name) {
      if let Ok(Ethernet(tx, rx)) = pnet_datalink::channel(&interface, Default::default()) {
        out_channels.push(tx);
        in_channels.push(rx);
        continue;
      }
    }
    return Err(io::Error::new(io::ErrorKind::InvalidInput, "wan interface"))
  }
  Ok((out_channels, in_channels))
}


