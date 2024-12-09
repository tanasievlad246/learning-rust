// src/bin/server.rs
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection established!");
                let mut buffer = [0; 1024];

                stream.read(&mut buffer).unwrap();
                println!("Received: {}", String::from_utf8_lossy(&buffer));

                stream.write(b"Hello from server!").unwrap();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
