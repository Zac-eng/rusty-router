use std::io;
use dotenv::dotenv;

pub mod interface;

fn main() -> io::Result<()> {
    dotenv().ok();
    let wan_intfs = vec!["WAN0_INTF", "WAN1_INTF"];

    let (lan_tx, lan_rx) = interface::lan::create_lan_channel()?;
    let (wan_txs, wan_rxs) = interface::wan::create_wan_channels(&wan_intfs)?;
    let bounding_channels = interface::wan::create_wan_bounding_channels::<&str>(&wan_intfs);
    Ok(())
}
