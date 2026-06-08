use std::{net::TcpListener, sync::{Arc, Mutex}};

use crate::{server::connection::Connection, store::Store};

const HOST: &str = "127.0.0.1";
const PORT: &str = "6379";

pub fn run(store: Arc<Mutex<Store>>){
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))
        .expect("Failed to bind address");

    println!("Server listening on {HOST}:{PORT}");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected!");
                
                let mut connection = Connection::new(stream, Arc::clone(&store));
                connection.process();
            }
            Err(e) => {
                eprintln!("Connection error: {e}");
            }
        }
    }
}