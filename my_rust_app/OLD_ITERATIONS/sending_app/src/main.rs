use tokio::net::TcpListener;

use std::io;

async fn process_socket<T>(socket: T) {
    // do work with socket here
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8182").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}