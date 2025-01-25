use openssl::symm::{Cipher, Crypter, Mode};

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
