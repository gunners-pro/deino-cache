pub mod ping;
pub mod echo;

pub enum Command {
    Unknown,
    Ping,
    MissingArg,
    Echo(String),
    Set(String, String),
    Get(String),
    Delete(String)
}

pub fn parser(input: &str) -> Command {
    if input == "PING" { Command::Ping }
    else if input.starts_with("ECHO") { 
        let echo_response: Vec<&str> = input.splitn(2, ' ').collect();        
        if echo_response.get(1).is_some() {
            Command::Echo(echo_response[1].to_string())
        } else {
            Command::MissingArg
        }
    }
    else if input.starts_with("SET") {
        let response: Vec<&str> = input.splitn(3, ' ').collect();       

        if response.len() > 2 {
            Command::Set(response[1].to_string(), response[2].to_string())
        } else {
            Command::MissingArg
        }
    }
    else if input.starts_with("GET") {
        let response: Vec<&str> = input.splitn(2, ' ').collect();
        if response.get(1).is_some() {
            Command::Get(response[1].to_string())
        } else {
            Command::MissingArg
        }
    }
    else if input.starts_with("DELETE") {
        let response: Vec<&str> = input.splitn(2, ' ').collect();
        if response.get(1).is_some() {
            Command::Delete(response[1].to_string())
        } else {
            Command::MissingArg
        }
    }
    else {
        Command::Unknown
    }
}