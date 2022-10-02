use std::str;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
#[tokio::main]
async fn main() {
    // The data type for storing in the information ARC is F_32
    let information = Arc::new(Mutex::new(0.00f32));

    //This is a very classic TCP port to use. Lots of the tutorials I followed to help create this used it as well
    let listener = TcpListener::bind("127.0.0.1:8181").await.unwrap();

    let vec2 = Arc::new(RwLock::new(vec![]));
    

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let vec1 = vec2.clone();

        let stream2 = TcpStream::connect("127.0.0.1:8182").await.unwrap();
        let information = information.clone();
        tokio::spawn(async move {
            handle_connection(stream,stream2,information, vec1.clone()).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream,mut stream2: TcpStream, information: Arc<Mutex<f32>>, vec1: Arc<RwLock<Vec<f32>>>) {

    let mut buffer = [0; 16];
    stream.read(&mut buffer).await.unwrap();

    // Making the buffer 
    let method_type = match str::from_utf8(&buffer[0..4]) {
        Ok(v) => v,
        // I pulled this error directly from a tokio tutorial example. I suck at including errors in my code so I had to google this
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let contents = match method_type {

        // TO DO finish GET METHOD
        "GET " => {

            // Learning what unwrap is is very useful. It returns the contained Values
            format!("{{\"sensor info\": {}}}", information.lock().unwrap())
        }
        "POST" => {

            let input: String = buffer[6..16]
                .iter()
                .take_while(|x| **x != 32u8)
                .map(|x| *x as char)
                .collect();
            let sensor = input.parse::<f32>().unwrap();

            // We need to Mutate the editable_users vec we created and need to await to write to it
            let mut editable_users = vec1.write().await;
            editable_users.push(sensor);
            
            // Figuring out how to format the Vec editable users to a string honestly kicked my butt
            format!(r"{:?}", editable_users)
        }
        _ => {
            // I find it funny that Rust has a macro rule called panic. Its Straight up classy.
            panic!("Invalid HTTP method!")
        }
    };

    let response = contents;
    // Flushing both streams is pretty important to avoid the buffer sending bad data which i think is called poisoned?
    stream2.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
    stream2.flush().await.unwrap();
}