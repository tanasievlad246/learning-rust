// src/bin/client.rs
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    println!("Enter your name: ");
    let user_name = get_user_input();

    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("Connected to server!");

            loop {
                let input = get_user_input();
                let message = format!("{}: {}", user_name.trim(), input.trim());
                stream.write(message.as_bytes()).unwrap();

                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();

                println!("{}", String::from_utf8_lossy(&buffer));
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input;
}
