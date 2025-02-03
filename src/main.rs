use std::{env, io, sync::{Arc, Mutex}, thread::{self, JoinHandle}};
use dotenv::dotenv;

mod interface;
mod napt;
mod scheduler;

use interface::{constructor::construct_interface, thread_funcs::{lan_thread_func, wan_thread_func}};
use napt::NAPTer;

fn main() -> io::Result<()> {
    dotenv().ok();

    let (lan_tx, mut lan_rx) = construct_interface(&env::var("LAN_INTF").unwrap(), "LAN_FIRSTHOP_MAC")?;
    let (wan0_tx, mut wan0_rx) = construct_interface(&env::var("WAN0_INTF").unwrap(), "WAN0_FIRSTHOP_MAC")?;
    let lan_tx_arc = Arc::new(Mutex::new(lan_tx));
    let arc0 = Arc::clone(&lan_tx_arc);
    let napter = Arc::new(Mutex::new(NAPTer::new(wan0_rx.ipv4_addr)));
    let arc_napt0 = napter.clone();
    let arc_napt1 = napter.clone();

    let mut handles: Vec<JoinHandle<io::Result<()>>> = Vec::new();
    handles.push(thread::spawn(move || {
        lan_thread_func(&mut lan_rx, &mut vec![wan0_tx], arc_napt0)
    }));
    handles.push(thread::spawn(move || {
        wan_thread_func(&mut wan0_rx, arc0, arc_napt1)
    }));
    for handle in handles {
        let _ = handle.join().unwrap()?;
    }
    Ok(())
}
