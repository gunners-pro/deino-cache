use std::{io::{BufReader, Read, Write}, net::TcpStream};

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
                (&self.stream)
                    .write_all(b"+PONG\r\n")
                    .unwrap();
            }

            if command.starts_with("ECHO") {
                let parts: Vec<&str> =
                    command.splitn(2, ' ').collect();

                if let Some(message) = parts.get(1) {
                    (&self.stream)
                        .write_all(message.as_bytes())
                        .unwrap();
                }
            }
        }
    }
}