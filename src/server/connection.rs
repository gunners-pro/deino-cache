use std::{io::{BufReader, Read, Write}, net::TcpStream, sync::{Arc, Mutex}};

use crate::{command::{Command, echo, parser, ping}, store::Store};

pub struct Connection {
    stream: TcpStream,
    store: Arc<Mutex<Store>>
}

impl Connection {
    pub fn new(stream: TcpStream, store: Arc<Mutex<Store>>) -> Self {
        Self { stream, store }
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
            let command = parser(command.trim());

            match command {
                Command::Ping => {
                    (&self.stream).write_all(ping::execute()).unwrap();
                }
                Command::Unknown => {
                    (&self.stream).write_all(b"Unknown").unwrap();
                },
                Command::MissingArg => {
                    (&self.stream).write_all(b"MissingArg").unwrap();
                },
                Command::Echo(res) => {
                    if let Some(message) = echo::execute(&res) {
                    (&self.stream).write_all(message.as_bytes()).unwrap();
                }
                },
                Command::Set(key, value) => {
                    self.store.lock().unwrap().set(key, value);
                },
                Command::Get(key) => {
                    if let Some(res) = self.store.lock().unwrap().get(key.as_str()) {
                        (&self.stream).write_all(res.as_bytes()).unwrap();
                    } else {
                        (&self.stream).write_all(b"key not found").unwrap();
                    }
                },
                Command::Delete(key) => {
                    self.store.lock().unwrap().delete(key.as_str());
                },
            }           
        }
    }
}