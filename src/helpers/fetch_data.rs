use error_chain::error_chain;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::fs::{ metadata, read_to_string };
use std::env;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

/**
 * Helper function to read or download crypto challenge exercise file
 * Usage: bytes_content: &[u8] = &get_challenge_file("4.txt");
 */
pub fn get_challenge_file(filename: &str) -> Vec<u8> {
  // create path for file
  let path = format!("{}{}{}", env::current_dir().unwrap().into_os_string().into_string().unwrap(), "/data/ex", filename);

  // check if file is already downloaded
  if _file_exists(path.as_str()) {
    // if downloaded read content and return as bytes
    return _read_bytes(path).as_bytes().to_vec();
  } else {
    // if not downloaded then download, create and return content
    let target = format!("{}{}", "https://cryptopals.com/static/challenge-data/", filename);

    let error = format!("Failed to download file from {}", target);
    let mut response = reqwest::blocking::get(target).expect(error.as_str());
    let mut body = String::new();
    response.read_to_string(&mut body).expect("Failed reading response");

    if response.status() != 200 {
      // if response is not 200 then body is not correct return empty string as bytes
      return "".as_bytes().to_vec();
    }

    let error = format!("Failed to create file {}", path);
    let mut file = File::create(path.clone()).expect(error.as_str());

    let error = format!("Failed to write file {}", path);
    file.write_all(body.as_bytes()).expect(error.as_str());

    return body.as_bytes().to_vec();
  }
}

/**
 * Private function to check if a file exists in provided path; used by exported get_challenge_file function
*/
fn _file_exists(path: &str) -> bool {
  return metadata(path).is_ok();
}

/**
 * Private function to read a file used by exported get_challenge_file function
*/
fn _read_bytes(path: String) -> String {
  return read_to_string(path)
    // .and_then(|res| Ok(res.replace("\n", "")))
    .expect("Error reading file")
}