use std::{env, io, thread::{self, JoinHandle}};
use pnet_datalink::{Channel::Ethernet, DataLinkReceiver, DataLinkSender};

pub fn create_lan_channel() -> io::Result<(Box<dyn DataLinkSender>, Box<dyn DataLinkReceiver>)> {
  let lan_intf = env::var("LAN_INTF").unwrap();
  if let Some(interface) = pnet_datalink::interfaces().into_iter().find(|intf| intf.name == lan_intf) {
    if let Ok(Ethernet(tx, rx)) = pnet_datalink::channel(&interface, Default::default()) {
      return Ok((tx, rx))
    }
  }
  Err(io::Error::new(io::ErrorKind::InvalidInput, "lan interface"))
}

pub fn lan_thread_func(mut lan_rx: Box<dyn DataLinkReceiver>, mut wan_txs: Vec<Box<dyn DataLinkSender>>) -> JoinHandle<io::Result<()>>{
  thread::spawn(move || {
    // let mut scheduler = 
    let mut buffer = lan_rx.next()?;
    
    Ok(())
  })
}
