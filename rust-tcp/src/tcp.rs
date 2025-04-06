use std::net::TcpStream;
use std::io::{Read, Write};

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024]; 
    stream.read(&mut buffer).expect("Failed to read from client"); 
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let response: &[u8] = "Hello, Client".as_bytes(); 
    stream.write(response).expect("Failed to write to response");
}
