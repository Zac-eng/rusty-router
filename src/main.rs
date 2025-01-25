mod interface;
use interface::lan_intf;

use std::{env, io, thread::{self, JoinHandle}};
use dotenv::dotenv;

fn main() -> io::Result<()> {
    dotenv().ok();

    let mut lan_intf = lan_intf::Lan::new(&env::var("LAN_INTF").expect("LAN_INTF not given"))?;
    let mut thread_handles: Vec<JoinHandle<io::Result<()>>> = Vec::new();
    // let mut interface = Interface::new(intf).unwrap();
    // println!("{:?}", interface.interface.mac.unwrap());
    thread_handles.push(thread::spawn(move || lan_intf.run()));
    for handle in thread_handles {
        handle.join();
    }
    Ok(())
}
