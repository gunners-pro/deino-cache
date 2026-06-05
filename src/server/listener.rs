use std::{net::TcpListener};

use crate::server::connection::Connection;

const HOST: &str = "127.0.0.1";
const PORT: &str = "6379";

pub fn run(){
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))
        .expect("Failed to bind address");

    println!("Server listening on {HOST}:{PORT}");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected!");
                
                let mut connection = Connection::new(stream);
                connection.process();
            }
            Err(e) => {
                eprintln!("Connection error: {e}");
            }
        }
    }
}