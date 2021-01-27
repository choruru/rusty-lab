use async_std::task;
use httpd::config::Config;

use async_std::fs;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::stream::StreamExt;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let config = Config::new("src/example_conf.yaml");
    println!("Reading config...\n{:?}", config);

    let listener = TcpListener::bind("localhost:10000").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            handle_connection(stream).await;
        })
        .await;
    println!("Shutting down.");
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/html/hello.html")
    } else if buf.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "src/html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/html/404.html")
    };

    let contents = fs::read_to_string(filename).await.unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
