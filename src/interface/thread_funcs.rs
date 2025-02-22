use std::{io, sync::Arc};
use std::sync::Mutex;

use crate::napt::NAPTer;
use crate::scheduler::RoundRobinScheduler;

use super::{IntfInput, IntfOutput};

pub fn lan_thread_func(
  lan_in: &mut IntfInput,
  wan_out: &mut Vec<IntfOutput>,
  napter: Arc<Mutex<NAPTer>>
) -> io::Result<()> {
  println!("lan thread spawn");
  let mut scheduler = RoundRobinScheduler::new(wan_out.len());
  loop {
    let ether_frame = lan_in.receive()?;
    if let Some(ip_packet) = lan_in.classify(ether_frame) {
      let out_intf = &mut wan_out[scheduler.next()];
      let mut napter = napter.lock().unwrap();
      if let Some(napted_packet) = napter.translate_outgoing(ip_packet) {
        if let Some(ether_to_send) = out_intf.ether_encap(napted_packet) {
          out_intf.send(ether_to_send)?;
        };
      };
    };
  }
}

pub fn wan_thread_func(
  wan_in: &mut IntfInput,
  lan_out_arc: Arc<Mutex<IntfOutput>>,
  napter: Arc<Mutex<NAPTer>>
) -> io::Result<()> {
  println!("wan thread spawn");
  loop {
    let ether_frame = wan_in.receive()?;
    if let Some(ip_packet) = wan_in.classify(ether_frame) {
      let napter = napter.lock().unwrap();
      if let Some(napted_packet) = napter.translate_incoming(ip_packet) {
        println!("here");
        let mut lan_out = lan_out_arc.lock().unwrap();
        if let Some(ether_to_send) = lan_out.ether_encap(napted_packet) {
          lan_out.send(ether_to_send)?;
          println!("sent");
        };
      };
    };
  }
}
