use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: \r\n{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        // 返回main.html
        let content = fs::read_to_string("main.html").unwrap();
        // let response = "HTTP/1.1 200 OK\r\n\r\n{}";
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // 返回404.html
        let content = fs::read_to_string("404.html").unwrap();
        // let response = "HTTP/1.1 200 OK\r\n\r\n{}";
        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}