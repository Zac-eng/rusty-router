mod crypto;

use std::io;
use std::io::Read;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (key, iv) = load_crypto_info()?;
    let data = b"Hello, OpenSSL in Rust!";
    let encrypted = crypto::encrypt_packet(data, &key, &iv)?;
    println!("Encrypted packet: {:?}", encrypted);
    let decrypted = crypto::decrypt_packet(&encrypted, &key, &iv)?;
    println!("Decrypted packet: {}", String::from_utf8(decrypted.clone())?);
    println!("len: before {}, after {}", decrypted.len(), encrypted.len());
    Ok(())
}

fn load_crypto_info() -> io::Result<([u8;32], [u8;16])> {
    let mut key_file = File::open("shared.key")?;
    let mut iv_file = File::open("init_val.txt")?;
    let mut key = [0u8;32];
    let mut iv = [0u8;16]; 
    key_file.read_exact(&mut key)?;
    iv_file.read_exact(&mut iv)?;
    return Ok((key, iv))
}
