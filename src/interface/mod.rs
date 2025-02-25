use std::{io, sync::mpsc::{self, Receiver, Sender}};

pub mod lan;
pub mod wan;
pub mod thread_funcs;

use lan::{LanOutput, LanInput};
use wan::{WanInput, WanOutput};

pub fn create_lan_intf() -> io::Result<(LanOutput, LanInput)> {
  lan::construct_lan_interface("LAN_INTF", "LAN_FIRSTHOP_MAC")
}

pub fn create_wan_intfs() -> io::Result<(Vec<WanOutput>, Vec<WanInput>)> {
  let intfs = ["WAN0_INTF", "WAN1_INTF"];
  let dstips = ["WAN0_DST_IP", "WAN1_DST_IP"];
  let dstmacs = ["WAN0_FIRSTHOP_MAC", "WAN1_FIRSTHOP_MAC"];
  let mut wan_outs: Vec<WanOutput> = Vec::new();
  let mut wan_ins: Vec<WanInput> = Vec::new();

  for i in 0..intfs.len() {
    let (output, input) = wan::construct_wan_interface(intfs[i], dstips[i], dstmacs[i])?;
    wan_outs.push(output);
    wan_ins.push(input);
  }
  Ok((wan_outs, wan_ins))
}

pub fn create_wan_channels(channel_size: usize) -> (Vec<Sender<Vec<u8>>>, Vec<Receiver<Vec<u8>>>) {
  let mut tx_vec: Vec<Sender<Vec<u8>>> = Vec::new();
  let mut rx_vec: Vec<Receiver<Vec<u8>>> = Vec::new();

  for _ in 0..channel_size {
    let (tx, rx) = mpsc::channel();
    tx_vec.push(tx);
    rx_vec.push(rx);
  }
  (tx_vec, rx_vec)
}
