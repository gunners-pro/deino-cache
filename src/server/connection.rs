use std::{io::{BufReader, Read, Write}, net::TcpStream};

use crate::command::{echo, ping};

pub struct Connection {
    stream: TcpStream
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn process(&mut self) {
        let mut reader = BufReader::new(&self.stream);

        loop {
            let mut buffer = [0u8; 1024];
            let bytes = match reader.read(&mut buffer) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Read error: {e}");
                    break;
                }
            };

            if bytes == 0 {
                println!("Client disconnected");
                break;
            }

            let command = String::from_utf8_lossy(&buffer[..bytes]);
            let command = command.trim();

            if command == "PING" {
                (&self.stream).write_all(ping::execute()).unwrap();
            } else if command.starts_with("ECHO") {
                if let Some(message) = echo::execute(command) {
                    (&self.stream).write_all(message.as_bytes()).unwrap();
                }
            }            
        }
    }
}