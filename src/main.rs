// use nickel::{HttpRouter, MiddlewareResult, Nickel, Request, Response};
mod logger;

fn main() {
    match logger::log_time("log.txt") {
        Ok(_) => println!("File created"),
        Err(_) => println!("Error: Could not create file")
    }
}
