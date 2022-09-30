use std::str;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // create balance wrapped in Arc and Mutex for cross thread safety
    let information = Arc::new(Mutex::new(0.00f32));
    let listener = TcpListener::bind("127.0.0.1:8181").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let information = information.clone();
        tokio::spawn(async move {
            handle_connection(stream, information).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream, information: Arc<Mutex<f32>>) {

    let mut buffer = [0; 16];
    stream.read(&mut buffer).await.unwrap();

    let method_type = match str::from_utf8(&buffer[0..4]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let contents = match method_type {
        "GET " => {

            format!("{{\"sensor info\": {}}}", information.lock().unwrap())
        }
        "POST" => {

            let input: String = buffer[6..16]
                .iter()
                .take_while(|x| **x != 32u8)
                .map(|x| *x as char)
                .collect();
            let sensor = input.parse::<f32>().unwrap();

            let mut locked_sensor: MutexGuard<f32> = information.lock().unwrap();
            *locked_sensor += sensor;
            format!("{{\"sensor info\": {}}}", locked_sensor)
        }
        _ => {
            panic!("Invalid HTTP method!")
        }
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}