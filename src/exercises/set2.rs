use crate::helpers::{pad_pkcs7, get_challenge_file, decrypt_cbc_mode};

/**
 * Implement PKCS#7 padding
 */
pub fn ex9() {
  println!("Ex9:");
  let message = "YELLOW SUBMARINE";
  let expected = "YELLOW SUBMARINE\x04\x04\x04\x04";

  let padded = pad_pkcs7(message, 20);
  println!("Expected equals result: {}", padded.eq(expected));
  println!("--");
}

/**
 * Implement CBC mode
 */
pub fn ex10() {
  let content = get_challenge_file("10.txt");
  let content = String::from_utf8(content).expect("err");
  let key = "YELLOW SUBMARINE";
  let iv = "\x00\x00\x00 &c";
  let result = decrypt_cbc_mode(&content, key, iv);

  println!("{}", result);
}