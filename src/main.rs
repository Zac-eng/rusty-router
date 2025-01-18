mod crypto;
mod capsule;
mod interface;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (key, iv) = crypto::load_crypto_info()?;
    let data = b"Hello, OpenSSL in Rust!";
    let encrypted = crypto::encrypt_packet(data, &key, &iv)?;
    println!("Encrypted packet: {:?}", encrypted);
    let decrypted = crypto::decrypt_packet(&encrypted, &key, &iv)?;
    println!("Decrypted packet: {}", String::from_utf8(decrypted.clone())?);
    println!("len: before {}, after {}", decrypted.len(), encrypted.len());
    Ok(())
}

// fn main() -> io::Result<()> {
//     let interface_name = "en0";
//     let mut intf = match pnet_datalink::interfaces().into_iter().find(|d| d.name == "en0") {
//         Some(intf) => intf,
//         None => return Err(io::Error::new(io::ErrorKind::NotFound, interface_name))
//     };
//     let mut interface = Interface::new(intf).unwrap();
//     println!("{:?}", interface.interface.mac.unwrap());
//     Ok(())
// }
