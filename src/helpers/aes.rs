use openssl::symm::{encrypt, decrypt};
use crate::helpers;

pub fn decrypt_aes_ecb(key: &str, bytes: &[u8]) -> Vec<u8> {
  let cipher = openssl::symm::Cipher::aes_128_ecb();
  let key = key.as_bytes();
  return decrypt(cipher, key, Some(key), &bytes).unwrap();
}

fn pad_pkcs7(message: &str, block_size: usize) -> String {
  let padding_size = block_size - message.len() % block_size;
  println!("{:?}", padding_size);
  let padding: String = (0..padding_size).map(|_| padding_size.to_string()).collect();
  let x =  format!("{:#2X?}", padding);
  println!("{}", x);
  let result = format!("{}{}", message, padding);
  println!("{}", result.as_bytes().len());
  return result;
}

// set2 e10
// let base64_bytes = read_bytes(path);
fn encrypt_aes_cbc(key: &str, data: &[u8], iv: &[u8]) -> Vec<u8> {
  let cipher = openssl::symm::Cipher::aes_128_ecb();
  let key = key.as_bytes();
  return encrypt(cipher, key, Some(iv), &data).unwrap();
}

fn encrypt_cbc_mode(message: &str, key_str: &str, iv_str: &str) -> String {
  let padded_msg = pad_pkcs7(message, 16);
  let msg_bytes = padded_msg.as_bytes();
  let iv = iv_str.as_bytes().to_vec();

  let mut enc_bytes: Vec<Vec<u8>> = Vec::new();

  (0..message.len()).step_by(16).for_each(|x| {
      let last = enc_bytes.last().unwrap_or(&iv);

      let xor_block = helpers::xor(last, &msg_bytes[x..x + 16]);
      let enc_result = encrypt_aes_cbc(key_str, &xor_block, &iv);
      enc_bytes.push(enc_result.into_iter().collect::<Vec<u8>>());
  });

  return hex::encode(enc_bytes.into_iter().flatten().collect::<Vec<u8>>());
}