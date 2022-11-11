/**
 * Converts hexadecimal string into base64
 */
pub fn hex_to_base64(hex_string: &str) -> String {
    
  let bytes_string = hex::decode(&hex_string).unwrap();
  
  base64::encode(bytes_string)
}

pub fn hex_decode(hex_string: &str) -> Vec<u8> {
  hex::decode(hex_string).unwrap()
}

pub fn hex_encode(hex_string: Vec<u8>) -> String {
  hex::encode(hex_string)
}

pub fn base64_decode(base64_str: &str) -> Vec<u8> {
  base64::decode(base64_str).unwrap()
}