use glob::Pattern;

use super::{
    database::{DataTypes, DatabaseOps},
    de::Query,
    ser,
};

#[derive(Debug)]
pub enum Command {
    Ping,
    Get,
    Echo,
    Set,
    Keys,
    Delete,
    Unknown,
}

impl Command {
    pub fn from_str(command: &str) -> Command {
        match command.to_lowercase().as_str() {
            "ping" => Command::Ping,
            "get" => Command::Get,
            "echo" => Command::Echo,
            "set" => Command::Set,
            "keys" => Command::Keys,
            "del" => Command::Delete,
            _ => Command::Unknown,
        }
    }

    pub fn create_response(&self, query: &Query) -> Vec<u8> {
        match self {
            Command::Ping => ser::str("PONG"),
            Command::Get => handle_get(query),
            Command::Echo => handle_echo(query),
            Command::Set => handle_set(query),
            Command::Keys => handle_keys(query),
            Command::Delete => handle_delete(query),
            Command::Unknown => ser::err(" Err Unknown command"),
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
    let mut db = DatabaseOps;
    let key = query.value.to_string();

    let data = &query.args;

    let default_value = "".to_string();
    let data = data.first().unwrap_or(&default_value);

    db.set(key, DataTypes::String(data.to_string()));

    ser::str("OK")
}

fn handle_get(query: &Query) -> Vec<u8> {
    let db = DatabaseOps;
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

fn handle_keys(query: &Query) -> Vec<u8> {
    let db = DatabaseOps;
    let keys = db.keys();
    let pattern = Pattern::new(&query.value).unwrap();

    if query.value.is_empty() {
        return ser::err("ERR wrong number of arguments for 'keys' command");
    }

    if keys.len() == 0 {
        return ser::nil();
    }

    if query.value.is_empty() {
        return ser::str_arr(&keys);
    }

    let keys: Vec<String> = keys
        .iter()
        .filter(|key| pattern.matches(key))
        .map(|key| key.to_string())
        .collect();

    ser::str_arr(&keys)
}

fn handle_delete(query: &Query) -> Vec<u8> {
    let db = DatabaseOps;
    let key = &query.value;
    let result = db.del(key.to_string());

    match result {
        Some(_) => ser::int(1),
        None => ser::int(0),
    }
}
