use std::{env, io, sync::{Arc, Mutex}, thread::{self, JoinHandle}};
use dotenv::dotenv;

mod interface;
mod napt;
mod crypto;
mod scheduler;
mod test;

// use interface::{constructor::construct_interface, thread_funcs::{lan_thread_func, wan_thread_func}};

use interface::{create_lan_intf, create_wan_channels, create_wan_intfs, thread_funcs::{lan_thread_func, wan_bounding_func, wan_intf_func}};
use napt::NAPTer;

fn main() -> io::Result<()> {
    dotenv().ok();
    println!("Rusty Bounding Router Running!!");

    let (mut lan_tx, mut lan_rx) = create_lan_intf()?;
    let (mut wan_txs, mut wan_rxs) = create_wan_intfs()?;
    let (mut wan_channel_txs, wan_channel_rxs) = create_wan_channels(wan_txs.len());

    let mut handles: Vec<JoinHandle<io::Result<()>>> = Vec::new();
    handles.push(thread::spawn(move || {
        lan_thread_func(&mut lan_rx, &mut wan_txs)
    }));
    for i in 0..wan_rxs.len() {
        let mut wan_rx = wan_rxs.remove(0);
        let wan_channel = wan_channel_txs.remove(0);
        handles.push(thread::spawn(move || {
            wan_intf_func(&mut wan_rx, &wan_channel)
        }));
    }
    handles.push(thread::spawn(move || {
        wan_bounding_func(&mut lan_tx, &wan_channel_rxs)
    }));
    for handle in handles {
        let _ = handle.join().unwrap()?;
    }
    Ok(())
}
