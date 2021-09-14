use std::{fs::File, io::Write, fs::OpenOptions};
use std::{io};

extern crate chrono;
use chrono::*;

fn create_file(filename: String) -> Result<File, io::Error> {
  let f =  OpenOptions::new().create(true).append(true).open(filename)?;
  Ok(f)
}

fn log_string(filename: String, string: &[u8]) -> std::io::Result<()> {
  let mut buffer = create_file(filename).unwrap();
  let mut pos = 0;

  while pos < string.len() {
      let bytes_written = buffer.write(&string[pos..])?;
      pos += bytes_written;
  }

  Ok(())
  
}

fn create_time_entry(entry_data: Option<String>) -> String {
  let local: DateTime<Local> = Local::now();
  let now = local.format("%a, %b %d %Y %I:%M:%S %p").to_string();
  match entry_data {
    Some(str) => [now, " - ".to_string(), str, "\n".to_string()].join(""),
    None => [now, "\n".to_string()].join("")
  }
}

pub fn log_time(str: String, token: Option<String>) -> io::Result<String> {
  let entry = create_time_entry(token);
  let bytes = entry.as_bytes();
  log_string(str, bytes)?;
  Ok(entry)
}