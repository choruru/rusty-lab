use httpd::threadpool::ThreadPool;
use httpd::config::Config;

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let config = Config::new("src/example_conf.yaml");
    println!("Reading config...\n{:?}", config);

    let listener = TcpListener::bind("localhost:10000").unwrap();
    let pool = ThreadPool::new(config.max_threads);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/html/hello.html")
    } else if buf.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "src/html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/html/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
