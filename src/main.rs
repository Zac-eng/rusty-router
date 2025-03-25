mod crypto;
mod socket;


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
