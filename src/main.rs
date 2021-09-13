use nickel::{MiddlewareResult, Nickel, Request, Response};
use tokio;
mod logger;

fn log(_req: &mut Request, res: Response) -> MiddlewareResult {
    match logger::log_time("log.txt") {
        Ok(s) => {
            let str = format!("Logged the time: {}", s);
            res.send(str)
        },
        Err(e) => {
            let str = format!("Error creating file{}", e);
            res.send(str)
        }
    }
}

#[tokio::main] 
async fn main() {
    let mut server = Nickel::new();
    server.utilize(log);
    // server.get("**", log);
    server.listen("127.0.0.1:6767").await.unwrap();

}
