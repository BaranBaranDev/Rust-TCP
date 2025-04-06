
mod tcp;
use std::net::TcpListener;
use tcp::handle_client; 
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").
    expect("Failed to bind to address");
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}