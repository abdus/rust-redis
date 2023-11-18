use super::{
    database::{DataTypes, Database},
    de::Query,
    ser,
};

#[derive(Debug)]
pub enum Command {
    Ping,
    Get,
    Unknown,
    Echo,
    Set,
}

impl Command {
    pub fn from_str(command: &str) -> Command {
        match command.to_lowercase().as_str() {
            "ping" => Command::Ping,
            "get" => Command::Get,
            "echo" => Command::Echo,
            "set" => Command::Set,
            c => {
                println!("Unknown command: {}", c);
                Command::Unknown
            }
        }
    }

    pub fn create_response(&self, query: &Query) -> Vec<u8> {
        match self {
            Command::Ping => ser::str("PONG"),
            Command::Get => handle_get(query),
            Command::Echo => handle_echo(query),
            Command::Set => handle_set(query),
            Command::Unknown => ser::str("Unknown command"),
        }
    }
}

fn handle_echo(query: &Query) -> Vec<u8> {
    let value = &query.value;
    let value = &value[..];
    let response = ser::bulk_str(&value);

    response
}

fn handle_set(query: &Query) -> Vec<u8> {
    let mut db = Database::get_instance();
    let key = query.value.to_string();

    let data = match &query.args {
        Some(data) => data,
        None => {
            let response = ser::err("ERR wrong number of arguments for 'SET' command");
            return response;
        }
    };

    let default_value = "".to_string();
    let data = data.first().unwrap_or(&default_value);

    db.set(key, DataTypes::String(data.to_string()));

    ser::str("OK")
}

fn handle_get(query: &Query) -> Vec<u8> {
    let db = Database::get_instance();
    let key = query.value.to_string();

    let data = db.get(key);

    let response = match data {
        Some(data) => match data {
            DataTypes::String(data) => ser::bulk_str(&data),
        },
        None => ser::nil(),
    };

    response
}
