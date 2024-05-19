use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::thread;

use itertools::Itertools;

fn handle_request(mut stream: TcpStream) {
    let buf = BufReader::new(stream.try_clone().unwrap());
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .take_while(|x| !x.is_empty())
        .collect_vec();

    let path = &lines[0].split(" ").collect_vec()[1].trim();

    if path == &"/" {
        stream
            .write(b"HTTP/1.1 200 OK\r\n\r\n")
            .expect("Write failed");
        return;
    }

    if path.starts_with("/echo") {
        let tokens = path.replace("/echo/", "");
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            tokens.len(),
            tokens.as_str()
        );
        stream.write(response.as_bytes()).expect("Write failed");
        return;
    }

    if path.starts_with("/user-agent") {
        let user_agent = lines
            .iter()
            .filter(|x| x.starts_with("User-Agent: "))
            .map(|x| x.replace("User-Agent: ", ""))
            .next()
            .expect("User agent not found");

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            user_agent.len(),
            user_agent.as_str()
        );
        stream.write(response.as_bytes()).expect("Write failed");
        return;
    }

    stream
        .write(b"HTTP/1.1 404 Not Found\r\n\r\n")
        .expect("Write failed");
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                println!("Received Request {:?}", s);
                std::thread::spawn(|| handle_request(s));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
