use std::time::Instant;
use std::error::Error;

use url::Url;
use console::style;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncReadExt;
use tokio::io::BufReader;
use regex::Regex;

async fn len<'a>(link: String) -> (String, usize, f64) {
    let now = Instant::now();
    let url = Url::parse(link.as_str()).unwrap();
    let host = url.host_str().unwrap();
    let path = url.path();

    let mut stream = TcpStream::connect(format!("{}:80", host).as_str()).await.unwrap();
    let req = vec![
        format!("GET {} HTTP/1.1", path).as_str(),
        format!("Host: {}", host).as_str(),
        "\r\n",
    ].join("\r\n");

    stream.write_all(req.as_bytes()).await.unwrap();

    let mut line = String::new();
    let mut reader = BufReader::new(stream);
    let mut len = 0;

    reader.read_line(&mut line).await.unwrap();

    while let Ok(size) = reader.read_line(&mut line).await {
        if size == 0 {
            break;
        }

        if line.starts_with("Content-Length: ") {
            let re = Regex::new(r"Content-Length: (\d+)").unwrap();
            len = re.captures(line.as_str()).unwrap()
                .get(1).unwrap()
                .as_str().parse::<usize>().unwrap();
        }

        if line == "\r\n" {
            break
        }

        line.truncate(0);
    }

    let mut content = vec![0; len];
    reader.read_exact(&mut content).await.unwrap();

    let content = String::from_utf8_lossy(&content);
    let re = Regex::new(r"/ebooks/(\d+)").unwrap();
    let len = re.captures_iter(&content).count();

    let author = url.path_segments().unwrap().last().unwrap();
    let took = now.elapsed().as_secs_f64();

    println!("{}\t{} seconds", link, took);

    (author.to_string(), len, took)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("[START]\n");

    let now = Instant::now();
    let mut tasks = vec![];
    let mut results = vec![];

    let authors = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
        'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    for author in authors {
        let url = format!("http://127.0.0.1/authors/{}", author);

        let task = tokio::spawn(async move {
            let (author, len, took) = len(url).await;
            (author, len, took)
        });

        tasks.push(task);
    }

    for task in tasks {
        let (author, len, took) = task.await?;

        results.push((author, len, took));
    }

    println!();

    for (author, len, took) in results {
        println!(
            "Author: {}\tNumber of articles: {:>6}\tTook: {} seconds",
            style(author).bold().red(),
            style(len).bold().green(),
            style(took).cyan()
        );
    }

    println!("\n > {} seconds", style(now.elapsed().as_secs_f64()).bold());

    Ok(())
}
