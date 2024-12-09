// src/bin/server.rs
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => loop {
                println!("New connection established!");
                let mut buffer = [0; 1024];

                loop {
                    stream.read(&mut buffer).unwrap();
                    println!("Received: {}", String::from_utf8_lossy(&buffer));

                    // write the received data back to the client
                    stream.write(&buffer).unwrap();
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
