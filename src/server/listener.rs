use std::{io::{BufReader, Read, Write}, net::TcpListener};

const HOST: &str = "127.0.0.1";
const PORT: &str = "6379";

pub fn run(){
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected!");
                let mut reader = BufReader::new(&stream);

                loop {
                    let mut buffer = [0u8; 1024];
                    let bytes = reader.read(&mut buffer).unwrap();
                    
                    if bytes == 0 {
                        break;
                    }

                    let command = String::from_utf8_lossy(&buffer[..bytes]);

                    if command.trim() == "PING" {
                        (&stream).write(b"+PONG\r\n").unwrap();
                    }
                    if command.trim().starts_with("ECHO") {
                        let res: Vec<&str> = command.splitn(2, " ").collect();
                        (&stream).write(res[1].as_bytes()).unwrap();
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}