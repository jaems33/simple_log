#[macro_use]
extern crate nickel;

use clap::{App, Arg};
use nickel::{Middleware, MiddlewareResult, Nickel, Request, Response};
use tokio;

mod logger;

struct LogData {
    pub filename: String,
    pub token: Option<String>,
}

fn logger_fn(req: &mut Request<LogData>, res: Response<LogData>) -> MiddlewareResult<LogData> {
    let filename = &req.server_data().filename;
    let token = &req.server_data().token;
    let output = match logger::log_time(filename.clone(), token.clone()) {
        Ok(s) => {
            format!("Logged the time: {}", s)
        }
        Err(e) => {
            format!("Error creating file{}", e)
        }
    };
    res.send(output.clone())
}

#[tokio::main]
async fn main() {
    let matches = App::new("Simple Log")
        .version("0.1")
        .author("James T. <james.teow33@gmail.com>")
        .about("A simple app to log running the app.")
        .arg(
            Arg::new("log_file")
                .about("Sets the filename to save logs to.")
                .long("logfile")
                .short('l')
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("auth_token")
                .about("Optional set token")
                .long("token")
                .short('t')
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let log_file = matches.value_of("log_file").unwrap().to_string();
    let token = matches.value_of("auth_token");

    let data = match token {
        Some(str) => LogData {
            filename: log_file,
            token: Option::Some(str.to_string()),
        },
        None => LogData {
            filename: log_file,
            token: Option::None,
        },
    };

    let mut server = Nickel::with_data(data);
    server.utilize(logger_fn);
    server.listen("127.0.0.1:6767").await.unwrap();
}
