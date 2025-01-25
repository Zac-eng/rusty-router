use std::io::{self, Read};
use std::fs::File;
use openssl::symm::{Cipher, Crypter, Mode};

pub fn load_crypto_info() -> io::Result<([u8;32], [u8;16])> {
  let mut key_file = File::open("shared.key")?;
  let mut iv_file = File::open("init_val.txt")?;
  let mut key = [0u8;32];
  let mut iv = [0u8;16]; 
  key_file.read_exact(&mut key)?;
  iv_file.read_exact(&mut iv)?;
  return Ok((key, iv))
}

pub fn encrypt_packet(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
  let cipher = Cipher::aes_256_cbc();
  let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))?;
  encrypter.pad(true);

  let mut ciphertext = vec![0; data.len() + cipher.block_size()];
  let mut count = encrypter.update(data, &mut ciphertext)?;
  count += encrypter.finalize(&mut ciphertext[count..])?;

  ciphertext.truncate(count);
  Ok(ciphertext)
}

pub fn decrypt_packet(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
  let cipher = Cipher::aes_256_cbc();
  let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))?;
  decrypter.pad(true);

  let mut plaintext = vec![0; data.len() + cipher.block_size()];
  let mut count = decrypter.update(data, &mut plaintext)?;
  count += decrypter.finalize(&mut plaintext[count..])?;

  plaintext.truncate(count);
  Ok(plaintext)
}
