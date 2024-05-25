use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use itertools::Itertools;

const RESPONSE_404: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
const RESPONSE_200: &str = "HTTP/1.1 200 OK";
const RESPONSE_201: &str = "HTTP/1.1 201 Created";

fn handle_request(mut stream: TcpStream, request: String, dir: String) {
    let (first_line, rest_lines) = request.split_once("\r\n").unwrap();
    let (method, rest) = first_line.split_once(" ").unwrap();

    let response = match method {
        "GET" => match rest.split_once(" ") {
            Some((path, _)) => {
                if path == "/" {
                    format!("{}\r\n\r\n", RESPONSE_200).to_string()
                } else if path.starts_with("/echo") {
                    let word = path.strip_prefix("/echo/").unwrap();
                    format!(
                        "{}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        RESPONSE_200,
                        word.len(),
                        word
                    )
                } else if path.starts_with("/user-agent") {
                    let user_agent = rest_lines
                        .split("\r\n")
                        .find(|line| line.starts_with("User-Agent"))
                        .unwrap()
                        .strip_prefix("User-Agent: ")
                        .unwrap();

                    format!(
                        "{}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                        RESPONSE_200,
                        user_agent.len(),
                        user_agent
                    )
                } else if path.starts_with("/files") {
                    let file = path.strip_prefix("/files").unwrap();

                    match std::fs::read_to_string(format!("{}/{}", dir, file)) {
                        Ok(content) => {
                            format!(
                                "{}\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                                RESPONSE_200,
                                content.len(),
                                content
                            )
                        }
                        Err(_) => RESPONSE_404.to_string(),
                    }
                } else {
                    RESPONSE_404.to_string()
                }
            }
            None => "HTTP/1.1 400 Bad Request\r\n\r\n".to_string(),
        },
        "POST" => match rest.split_once(" ") {
            Some((path, _)) => {
                if path.starts_with("/files") {
                    let fname = path.strip_prefix("/files").unwrap();
                    let mut file = File::create(format!("{}/{}", dir.to_owned(), fname)).unwrap();

                    let content = rest_lines
                        .split_once("\r\n\r\n").unwrap().1;

                    dbg!(content);

                    file.write_all(content.as_bytes()).unwrap();

                    format!("{}\r\n\r\n", RESPONSE_201)

                } else {
                    RESPONSE_404.to_string()
                }
            }
            None => "HTTP/1.1 400 Bad Request\r\n\r\n".to_string(),
        },
        _ => "HTTP/1.1 405 Method Not Allowed\r\n\r\n".to_string(),
    };

    stream.write(response.as_bytes()).unwrap();
}

fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let args = env::args().collect_vec();

    let dir = if args.len() > 1 && args[1] == "--directory" {
        args[2].clone()
    } else {
        String::from("")
    };

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                println!("new client!");
                let mut request = [0_u8; 1024];
                let bytes = s.read(&mut request).unwrap();
                let request = String::from_utf8_lossy(&request[..bytes]).into_owned();
                dbg!(&request);

                handle_request(s, request, dir.clone());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
