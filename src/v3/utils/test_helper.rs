#[cfg(test)]
use std::fs::File;
#[cfg(test)]
use std::io::Read;

#[cfg(test)]
pub fn file_to_string(path: &str) -> String {
  let mut response = File::open(path).unwrap();
  let mut body = String::new();
  response.read_to_string(&mut body).unwrap();

  body
}
