use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use itertools::Itertools;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                let buf = BufReader::new(s.try_clone().unwrap());
                let lines = buf
                    .lines()
                    .map(|line| line.unwrap())
                    .take_while(|x| !x.is_empty())
                    .collect_vec();

                println!("{:?}", lines);

                let path = &lines[0].split(" ").collect_vec()[1].trim();
                println!("Path: {:?}", path);

                if path.starts_with("/echo/") {
                    let tokens = path.replace("/echo/", "");
                    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", tokens.len(), tokens.as_str());
                    s.write(response.as_bytes()).expect("Write failed");
                    continue;
                }
                s.write(b"HTTP/1.1 404 Not Found\r\n\r\n")
                    .expect("Write failed");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
