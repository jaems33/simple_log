use std::{fs::File, io::Write};
use std::io;
use nickel::{HttpRouter, MiddlewareResult, Nickel, Request, Response};

fn log_something(filename: &'static str, string: &[u8]) -> std::io::Result<()> {
    let mut buffer = create_file(filename).unwrap();
    let mut pos = 0;

    // buffer.write_all(string)?;
    while pos < string.len() {
        let bytes_written = buffer.write(&string[pos..])?;
        pos += bytes_written;
    }

    Ok(())
    
    // result.write_all(string);
}

fn create_file(filename: &'static str) -> Result<File, io::Error> {
    let f = File::create(filename)?;
    Ok(f)
}

// #[tokio::main]
fn main() {
    // let mut server = Nickel::new();
    // server.get("**", hello_world);
    // server.listen("127.0.0.1:6767").await.unwrap();
    // match 
    log_something("example.txt", b"hello world");
}
