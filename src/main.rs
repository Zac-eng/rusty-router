mod interface;
mod crypto;
mod capsule;
mod arp_handler;

use interface::lan_intf;
use std::{env, io, thread::{self, JoinHandle}};
use dotenv::dotenv;

fn main() -> io::Result<()> {
    dotenv().ok();

    let mut lan_intf = lan_intf::Lan::new(&env::var("LAN_INTF").expect("LAN_INTF not given"))?;
    let mut wan_intf = lan_intf::Lan::new(&env::var("LOCAL_WAN_INTF0").expect("WAN_INTF not given"))?;
    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();
    // let mut interface = Interface::new(intf).unwrap();
    thread_handles.push(thread::spawn(move || lan_intf.run()));
    for handle in thread_handles {
        let result = handle.join();
    }
    Ok(())
}
