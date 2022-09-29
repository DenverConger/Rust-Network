use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    // ignore the Result
    let _ = stream.write(&[0]);
    let _ = stream.read(&mut [0; 128]); // ignore here too
    std::process::exit(0);
} // the stream is closed here