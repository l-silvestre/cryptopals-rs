use crate::helpers::base64_decode;
use crate::helpers::decrypt_aes_ecb;
use crate::helpers::hex_decode;
use crate::helpers::hex_to_base64;
use crate::helpers::hex_encode;
use crate::helpers::xor;
use crate::helpers::repeating_key_xor;
use crate::helpers::calc_letter_freq_score;
use crate::helpers::get_challenge_file;
use crate::helpers::calc_avg_edit_dist;
use std::collections::HashSet;
use std::io::BufRead;
use std::str;

/**
 * Convert hex to base64
 */
pub fn ex1() {
  println!("Ex1:");
  let original_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
  let converted_tring = hex_to_base64(original_string);
  let expected_string = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

  println!("Result string is: {}", converted_tring);
  println!("Original String equals Expected String: {}", converted_tring.eq(expected_string));
  println!("--");
}

/**
 * Fixed XOR
 */
pub fn ex2() {
  println!("Ex2:");
  let original_payload = "1c0111001f010100061a024b53535009181c";
  let noise = "686974207468652062756c6c277320657965";
  let expected_result = "746865206b696420646f6e277420706c6179";

  let decoded_payload = hex_decode(original_payload);
  let decoded_noise = hex_decode(noise);

  let xored = xor(&decoded_payload, &decoded_noise);
  let str_result = hex_encode(xored);
  println!("{}", str_result);
  println!("Original equals Expected: {}", str_result.eq(expected_result));
  println!("--");
}

/**
 * Single-byte XOR Cypher
 */
pub fn ex3() {
  println!("Ex3:");
  let message = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
  let cipher_bytes = hex_decode(message);

  let mut key_byte: u8;

  let mut message = String::new();
  let mut best_score = f64::MIN;

  for c in 0..=255 {
    key_byte = c as u8;

    let msg_bytes: Vec<u8> = cipher_bytes.iter().map(|&b| b ^ key_byte).collect();

    let msg = String::from_utf8_lossy(&msg_bytes);
    let score = calc_letter_freq_score(&msg);

    if score > best_score {
        best_score = score;
        message = String::from(msg);
    }
  }

  println!("{}", message);
  println!("--");
}

/**
 * Detect Single-Character XOR
 */
pub fn ex4() {
  println!("Ex4:");
  let content = get_challenge_file("4.txt");
  let content = String::from_utf8(content).expect("err").replace('\n', "").as_bytes().to_vec();
  let lines = content.chunks(60)
    .map(str::from_utf8)
    .collect::<Result<Vec<&str>, _>>()
    .unwrap();
  
  let mut message = String::new();
  let mut best_score: f64 = f64::MIN;
  let mut key: u16;
  for line in lines {
    for c in 0..255 {
      key = c as u16;

      let msg_bytes: Vec<u16> = hex_decode(line.trim())
        .iter()
        .map(|&b| (b as u16) ^ key)
        .collect();

      let msg = String::from_utf16(&msg_bytes).unwrap();
      let score = calc_letter_freq_score(&msg);

      if score > best_score {
        best_score = score;
        message = msg;
      }
    }
  }

  println!("{}", message);
  println!("--");
}

/**
 * Implement Repeating-key XOR
 */
pub fn ex5() {
  println!("Ex5:");
  let message = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
  let key = "ICE";
  let expected_str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

  let result = repeating_key_xor(message, key);
  let str_result = hex_encode(result);

  println!("{}", str_result);
  println!("Original equals Expected: {}", str_result.eq(expected_str));
  println!("--");
}

/**
 * Break Repeating-key XOR
 */
pub fn ex6() {
  println!("Ex6:");
  let content = get_challenge_file("6.txt");
  let content = base64_decode(&String::from_utf8(content).expect("err").replace('\n', ""));
  // println!("{}", hamming_distance_bytes("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()));
  let mut edit_dist: Vec<(usize, f64)> = Vec::new();

  for key_sz in 2..=40 {
    let dist = calc_avg_edit_dist(key_sz, &content);
    edit_dist.push((key_sz, dist));
  }

  // Extract the shortest distance key
  edit_dist.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());
  let key_sz = edit_dist.pop().map(|x| x.0).unwrap();

  // divide cyphertext into blocks of keysize
  let blocks = content.chunks_exact(key_sz);
  let mut full_key: Vec<u16> = Vec::new();
  // transpose blocks, for each chunk take nth byte of each chunk
  for (index, x) in blocks.enumerate() {
    if index >= x.len() {
      break;
    }
    let block: Vec<u8> = content
      .chunks_exact(key_sz)
      .map(|x| x[index])
      .collect();
    let mut block_best_score: f64 = f64::MIN;
    let mut key_str: u16 = 0;
    let mut key: u16;
    for c in 0..255 {
      key = c as u16;
      let msg_bytes: Vec<u16> = block
        .iter()
        .map(|&b| (b as u16) ^ key)
        .collect();

      let msg = String::from_utf16(&msg_bytes).unwrap();
      let score = calc_letter_freq_score(&msg);

      if score > block_best_score {
        block_best_score = score;
        key_str = key; 
      }
    }
    full_key.push(key_str);
  }
  let key_str = String::from_utf16(&full_key).expect("error");
  let content_str = String::from_utf8(content).expect("err");
  let result = repeating_key_xor(&content_str, &key_str);
  let str_result = String::from_utf8(result).expect("error");
  println!("{}", str_result);
  println!("--")
}

/**
 * AES in ECB mode
 */
pub fn ex7() {
  println!("Ex7:");
  let key ="YELLOW SUBMARINE";
  let content = get_challenge_file("7.txt");
  let content = base64_decode(&String::from_utf8(content).expect("err").replace('\n', ""));

  let res = decrypt_aes_ecb(key, &content);

  println!("{}", String::from_utf8_lossy(&res));
}

/**
 * Detect AES in ECB mode
 */
pub fn ex8() {
  println!("Ex8:");
  let content = get_challenge_file("8.txt");

  let mut i_line: usize = 0;
  let mut max_identical_blocks: usize = 0;

  let mut n_identical_blocks: usize;
  for (i, line) in content.lines().enumerate() {
    let hex = line.unwrap();
    // Hex line to bytes vec
    let bytes = hex::decode(hex).unwrap();

    // Divide bytes into 16 byte blocks (&[u8] blocks)
    let blocks: Vec<_> = bytes.chunks_exact(16).collect();

    // Get unique blocks
    let unique_blocks: HashSet<&[u8]> = blocks.iter().cloned().collect();

    // No. of identical blocks detected
    n_identical_blocks = blocks.len() - unique_blocks.len();

    // Cipher containing most identical blocks is more likely to be
    // ECB mode encrypted
    if n_identical_blocks > max_identical_blocks {
        max_identical_blocks = n_identical_blocks;
        i_line = i;
    }
  }
  println!("{}", i_line);

  println!("--");
}