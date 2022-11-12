const LETTER_FREQ: [f64; 27] = [
  0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
  0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
  0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
  0.00978, 0.02360, 0.00150, 0.01974, 0.00074, 0.19181, // V-Z & space char
];

/**
 * Function to xor to pieces of bytes
 */
pub fn xor(buffer1: &[u8], buffer2: &[u8]) -> Vec<u8> {
  buffer1
    .iter()
    .zip(buffer2.iter())
    .map(|(&b1, &b2)| b1 ^ b2)
    .collect()
}

pub fn repeating_key_xor(message: &str, key: &str) -> Vec<u8> {
  let key_seq: String = key.chars().cycle().take(message.len()).collect::<String>();
  let key_bytes = key_seq.as_bytes();
  let msg_bytes = message.as_bytes();

  xor(msg_bytes, key_bytes)
}

pub fn calc_letter_freq_score(s: &str) -> f64 {
  let mut counts = vec![0_u32; 27];
  let mut score: f64 = 0_f64;

  s.chars().for_each(|c| match c {
    'a'..='z' => {
      counts[c as usize - 97] += 1;
    }
    'A'..='Z' => {
      counts[c as usize - 65] += 1;
    }
    ' ' => counts[26] += 1,
    _ => {}
  });

  for i in 0..27 {
    score += (counts[i] as f64) * LETTER_FREQ[i];
  }

  score
}

pub fn hamming_distance_bytes(b1: &[u8], b2: &[u8]) -> u32 {
  if b1.len() != b2.len() {
    panic!("Unequal byte slices!");
  }

  // iterate over first string bytes and join with second string bytes
  // for each pair start with distance 0 and apply function 
  b1.iter().zip(b2.iter()).fold(0_u32, |dist, (x1, x2)| {
    // format bytes as a string of bits
    let bin1 = format!("{:08b}", x1);
    let bin2 = format!("{:08b}", x2);

    // for each iteration of pair add previous distance with new distance calculation
    // new distance calculation takes the chars of bits in first string,
    // joins with second string and for each pair compares the value
    // if value is same then just return previous distance othewise add 1 bit of distance
    
    dist + bin1
      .chars()
      .zip(bin2.chars())
      .fold(0_u32, |d, (ch1, ch2)| if ch1 == ch2 { d } else { d + 1 })
  })
}

pub fn calc_avg_edit_dist(key_sz: usize, txt_bytes: &[u8]) -> f64 {
  let len = txt_bytes.len();
  let mut i: usize = 0;
  let mut dist_sum = 0;
  let mut block1;
  let mut block2;

  loop {
    if i * 2 * key_sz >= len {
      break;
    }

    block1 = &txt_bytes[i * key_sz..(i + 1) * key_sz];
    block2 = &txt_bytes[(i + 1) * key_sz..(i + 2) * key_sz];

    dist_sum += hamming_distance_bytes(block1, block2) / (key_sz as u32);

    i += 1;
  }

  (dist_sum as f64) / (i as f64 + 1.0)
}