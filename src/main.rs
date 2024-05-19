use std::io::Write;
use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("accepted new connection");
                s.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("Write failed");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
