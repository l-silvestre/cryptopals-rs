use openssl::symm::{encrypt, decrypt};
use crate::helpers;

pub fn decrypt_aes_ecb(key: &str, bytes: &[u8]) -> Vec<u8> {
  let cipher = openssl::symm::Cipher::aes_128_ecb();
  let key = key.as_bytes();
  
  decrypt(cipher, key, Some(key), bytes).unwrap()
}

pub fn pad_pkcs7(message: &str, block_size: usize) -> String {
  let padding_size = block_size - message.len() % block_size;
  let padding_char = padding_size as u8 as char;
  let padding: String = (0..padding_size).map(|_| padding_char).collect();

  format!("{}{}", message, padding)
}

pub fn encrypt_cbc_mode(message: &str, key_str: &str, iv_str: &str) -> String {
  let padded_msg = pad_pkcs7(message, 16);
  let msg_bytes = padded_msg.as_bytes();
  let iv = iv_str.as_bytes().to_vec();

  let mut enc_bytes: Vec<Vec<u8>> = Vec::new();

  (0..message.len()).step_by(16).for_each(|x| {
    let last = enc_bytes.last().unwrap_or(&iv);

    let xor_block = helpers::xor(last, &msg_bytes[x..x + 16]);
    let enc_result = _encrypt_aes_cbc(key_str, &xor_block, &iv);
    enc_bytes.push(enc_result.into_iter().collect::<Vec<u8>>());
  });

  hex::encode(enc_bytes.into_iter().flatten().collect::<Vec<u8>>())
}

pub fn decrypt_cbc_mode(message: &str, key_str: &str, iv_str: &str) -> String {
  let encrypted_bytes = message.as_bytes();
  // let key = key_str.as_bytes();
  let iv = iv_str.as_bytes().to_vec();

  let mut decrypted_blocks: Vec<Vec<u8>> = Vec::new();

  (0..message.len()).step_by(16).for_each(|x| {
    // Take last of encrypted block or IV in case of first block iteration
    let last = if x == 0 {
      &iv
    } else {
      &encrypted_bytes[x - 16..x]
    };
    let decrypted_block = _decrypt_aes_cbc(key_str, &encrypted_bytes[x..x + 16], &iv);
    let decrypted_block = decrypted_block.into_iter().collect::<Vec<u8>>();

    let xor_block = helpers::xor(last, &decrypted_block);
    decrypted_blocks.push(xor_block);
  });

  let padding_byte = *decrypted_blocks.last().unwrap().last().unwrap() as usize;
  decrypted_blocks
    .into_iter()
    .flatten()
    .take(encrypted_bytes.len() - padding_byte)
    .map(|x| x as char)
    .collect::<String>()
}

fn _encrypt_aes_cbc(key: &str, data: &[u8], iv: &[u8]) -> Vec<u8> {
  let cipher = openssl::symm::Cipher::aes_128_cbc();
  let key = key.as_bytes();

  encrypt(cipher, key, Some(iv), data).unwrap()
}

fn _decrypt_aes_cbc(key: &str, data: &[u8], iv: &[u8]) -> Vec<u8> {
  let cipher = openssl::symm::Cipher::aes_128_cbc();
  let key = key.as_bytes();

  decrypt(cipher, key, Some(iv), data).unwrap()
}