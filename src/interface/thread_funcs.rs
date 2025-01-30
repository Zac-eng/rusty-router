use std::{io, sync::Arc};
use std::sync::Mutex;
use crate::scheduler::RoundRobinScheduler;

use super::{IntfInput, IntfOutput};

pub fn lan_thread_func(
  lan_in: &mut IntfInput,
  wan_out: &mut Vec<IntfOutput>,
) -> io::Result<()> {
  let mut scheduler = RoundRobinScheduler::new(wan_out.len());
  loop {
    let ether_frame = lan_in.receive()?;
    let ip_packet = match lan_in.classify(ether_frame) {
      Some(packet) => packet,
      None => continue,
    };
    let out_intf = &mut wan_out[scheduler.next()];
    let ether_to_send = match out_intf.ether_encap(ip_packet) {
      Some(frame) => frame,
      None => continue,
    };
    out_intf.send(ether_to_send)?;
  }
}

pub fn wan_thread_func(
  wan_in: &mut IntfInput,
  lan_out_arc: Arc<Mutex<IntfOutput>>,
) -> io::Result<()> {
  loop {
    let ether_frame = wan_in.receive()?;
    let ip_packet = match wan_in.classify(ether_frame) {
      Some(packet) => packet,
      None => continue,
    };
    let mut lan_out = lan_out_arc.lock().unwrap();
    let ether_to_send = match lan_out.ether_encap(ip_packet) {
      // to unlock mutex asap, put None handling beforehand
      None => continue,
      Some(frame) => frame
    };
    lan_out.send(ether_to_send)?;
    return Ok(())
  }
}
