mod interface;

use std::io;
use interface::Interface;

fn main() -> io::Result<()> {
    let interface_name = "en0";
    let mut intf = match pnet_datalink::interfaces().into_iter().find(|d| d.name == "en0") {
        Some(intf) => intf,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, interface_name))
    };
    let mut interface = Interface::new(intf).unwrap();
    println!("{:?}", interface.interface.mac.unwrap());
    Ok(())
}
