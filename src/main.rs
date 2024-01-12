use messenger_dog::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:0")?;
    run(listener)?.await
}
