use std::net::{TcpListener, TcpStream};

fn handle_client(_stream: TcpStream) {
  println!("receive something");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}