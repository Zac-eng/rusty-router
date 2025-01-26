mod interface;
mod crypto;
mod capsule;
mod arp_handler;

use interface::{lan_intf, open_ethernet_channel};
use std::{env, io, thread::{self, JoinHandle}};
use dotenv::dotenv;

fn main() -> io::Result<()> {
    dotenv().ok();

    // // (lan_rx, lan_tx) = open_ethernet_channel(interface)

    // let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();
    // // let mut interface = Interface::new(intf).unwrap();
    // thread_handles.push(thread::spawn(move || lan_intf.run()));
    // for handle in thread_handles {
    //     let result = handle.join();
    // }
    Ok(())
}
