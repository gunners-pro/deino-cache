use std::sync::{Arc, Mutex};

use crate::store::Store;

mod server;
mod command;
mod store;

fn main() {
    let store = Arc::new(Mutex::new(Store::new()));
    server::run(store);    
}
