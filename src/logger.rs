use std::{fs::File, io::Write, fs::OpenOptions};
use std::io;

extern crate chrono;
use chrono::*;

fn create_file(filename: &'static str) -> Result<File, io::Error> {
  let f =  OpenOptions::new().append(true).open(filename)?;
  Ok(f)
}

fn log_string(filename: &'static str, string: &[u8]) -> std::io::Result<()> {
  let mut buffer = create_file(filename).unwrap();
  let mut pos = 0;

  while pos < string.len() {
      let bytes_written = buffer.write(&string[pos..])?;
      pos += bytes_written;
  }

  Ok(())
  
}

fn create_time_entry() -> String {
  let local: DateTime<Local> = Local::now();
  local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string()
}

pub fn log_time(str: &'static str) -> io::Result<String> {
  let entry = create_time_entry();
  let bytes = entry.as_bytes();
  log_string(str, bytes)?;
  Ok(entry)
}