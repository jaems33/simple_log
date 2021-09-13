#[macro_use]
extern crate nickel;

use nickel::{Middleware, MiddlewareResult, Nickel, Request, Response};
use tokio;
use clap::{Arg, App};

mod logger;

fn logger_fn(req: &mut Request<String>, res: Response<String>) -> MiddlewareResult<String> {
    let str = req.server_data().to_string();
    let output = match logger::log_time(str) {
            Ok(s) => {
                format!("Logged the time: {}", s)
            },
            Err(e) => {
                format!("Error creating file{}", e)
            }
        };
    res.send(output.clone())
}


#[tokio::main] 
async fn main(){

    let matches = App::new("Simple Log")
        .version("0.1")
        .author("James T. <james.teow33@gmail.com>")
        .about("A simple app to log running the app.")
        .arg(Arg::new("log_file")
            .about("Sets the filename to save logs to.")
            .long("logfile")
            .short('l')
            .takes_value(true)
            .required(true))
        .arg(Arg::new("auth_token")
            .about("Optional set token")
            .long("token")
            .short('t')
            .takes_value(true)
            .required(false))
        .get_matches();
    
    let log_file = matches.value_of("log_file").unwrap().to_string();
    let mut server = Nickel::with_data(log_file);
    server.utilize(logger_fn);
    server.listen("127.0.0.1:6767").await.unwrap();
    
}