use std::fs;
use std::io::Read;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:1337") {
        Ok(l) => l,
        Err(e) => {
            println!("Error when starting server {}", e);
            return;
        }
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Buffer for reading incoming data from the TCP stream
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let expected_get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(expected_get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUDN", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    // Flush will wait until all bytes are written into the connection
    stream.flush().unwrap();
}
