// src/bin/client.rs
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("Connected to server!");

            stream.write(b"Hello from client!").unwrap();

            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();

            println!("Received: {}", String::from_utf8_lossy(&buffer));
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}
