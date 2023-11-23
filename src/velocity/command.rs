use glob::Pattern;

use super::{
    database::{DataTypes, DatabaseOps},
    query::Query,
    serializer,
};

#[derive(Debug)]
pub struct SetCommandOpts {
    ex: Option<i64>,   // expiry time in second
    px: Option<i64>,   // expiry time in millisecond
    nx: Option<bool>,  // only set the key if it does not already exist
    xx: Option<bool>,  // only set the key if it already exist
    exat: Option<i64>, // expiry unix time in second
    pxat: Option<i64>, // expiry unix time in millisecond
}

impl SetCommandOpts {
    pub fn new() -> SetCommandOpts {
        SetCommandOpts {
            ex: None,
            px: None,
            nx: None,
            xx: None,
            exat: None,
            pxat: None,
        }
    }
}

#[derive(Debug)]
pub struct SetCommandParseErr(String);

#[derive(Debug)]
pub enum Command {
    Ping,
    Get,
    Echo,
    Set,
    Keys,
    Delete,
    Exists,
    Incr,
    Decr,
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
            "exists" => Command::Exists,
            "incr" => Command::Incr,
            "decr" => Command::Decr,
            _ => Command::Unknown,
        }
    }

    pub fn create_response(&self, query: &Query) -> Vec<u8> {
        match self {
            Command::Ping => serializer::str("PONG"),
            Command::Get => handle_get(query),
            Command::Echo => handle_echo(query),
            Command::Set => handle_set(query),
            Command::Keys => handle_keys(query),
            Command::Delete => handle_delete(query),
            Command::Exists => handle_exists(query),
            Command::Incr => handle_incr_decr(query, IncrDecrOpts::Incr),
            Command::Decr => handle_incr_decr(query, IncrDecrOpts::Decr),
            Command::Unknown => serializer::err("Err Unknown command"),
        }
    }
}

fn handle_echo(query: &Query) -> Vec<u8> {
    let value = &query.command_str;
    let value = &value[..];
    let response = serializer::bulk_str(&value);

    response
}

fn parse_set_args(args: &Vec<String>) -> Result<SetCommandOpts, SetCommandParseErr> {
    let mut args = args.iter();
    let mut set_command = SetCommandOpts::new();

    //if current_arg.is_none() {
    //return Ok(SetCommandOpts::new());
    //}

    while let Some(current_arg) = args.next() {
        let current_arg = current_arg.to_uppercase();
        let current_arg = current_arg.as_str();

        println!("current_arg is {}", current_arg);

        match current_arg {
            "EX" => {
                if set_command.px.is_some()
                    || set_command.pxat.is_some()
                    || set_command.exat.is_some()
                {
                    let msg = "EX cannot be paired with other expiry commands".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                let value = args.next();

                if value.is_none() {
                    let msg = "No value passed for EX".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                let value = value.unwrap().parse::<i64>();

                if value.is_err() {
                    let msg = "Invalid value for EX".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                set_command.ex = Some(value.unwrap());
            }

            "PX" => {
                if set_command.px.is_some()
                    || set_command.pxat.is_some()
                    || set_command.exat.is_some()
                {
                    let msg = "PX cannot be paired with other expiry commands".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                let value = args.next();

                if value.is_none() {
                    let msg = "No value passed for PX".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                let value = value.unwrap().parse::<i64>();

                if value.is_err() {
                    let msg = "Invalid value for PX".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                set_command.px = Some(value.unwrap());
            }

            "NX" => {
                if set_command.xx.is_some() {
                    let msg = "NX cannot be paired with XX".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                set_command.nx = Some(true);
            }

            "XX" => {
                if set_command.nx.is_some() {
                    let msg = "XX cannot be paired with NX".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                set_command.xx = Some(true);
            }

            "EXAT" => {
                if set_command.px.is_some()
                    || set_command.pxat.is_some()
                    || set_command.ex.is_some()
                {
                    let msg = "EXAT cannot be paired with other expiry commands".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                let value = args.next();

                if value.is_none() {
                    let msg = "No value passed for EXAT".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                let value = value.unwrap().parse::<i64>();

                if value.is_err() {
                    let msg = "Invalid value for EXAT".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                set_command.exat = Some(value.unwrap());
            }

            "PXAT" => {
                let value = args.next();

                if value.is_none() {
                    let msg = "No value passed for PXAT".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                let value = value.unwrap().parse::<i64>();

                if value.is_err() {
                    let msg = "Invalid value for PXAT".to_string();
                    return Err(SetCommandParseErr(msg));
                }

                set_command.pxat = Some(value.unwrap());
            }

            _ => {
                let msg = format!("Invalid modifier: {}", current_arg);
                return Err(SetCommandParseErr(msg));
            }
        }
    }

    return Ok(set_command);
}

fn handle_set(query: &Query) -> Vec<u8> {
    let mut db = DatabaseOps;
    let key = query.command_str.to_string();

    let data = &query.args;

    let default_value = "".to_string();
    let data = data.first().unwrap_or(&default_value);

    // in `args` property, anything after the first element are the modifiers
    // for the command. For example:
    //          SET key value EX 10
    //              SET: the command
    //              key: identifier
    //              value: the value to be stored
    //              EX: modifier
    //              10: the value of the modifier

    let args = query
        .args
        .iter()
        .map(|arg| arg.to_string())
        .skip(1) // first element is the value to be stored. so remove it
        .collect::<Vec<String>>();

    let parsed = parse_set_args(&args);

    match parsed {
        Ok(parsed) => {
            if parsed.ex.is_some() {
                let ex = parsed.ex.unwrap();
                let unix_time = chrono::Utc::now().timestamp() + ex;
                db.expire(key.clone(), unix_time + ex);
            } else if parsed.px.is_some() {
                let px = parsed.px.unwrap();
                let unix_time = chrono::Utc::now().timestamp_millis() + (px as i64 / 1000);
                db.expire(key.clone(), unix_time);
            } else if parsed.exat.is_some() {
                let exat = parsed.exat.unwrap();
                db.expire(key.clone(), exat);
            } else if parsed.pxat.is_some() {
                let pxat = parsed.pxat.unwrap();
                db.expire(key.clone(), pxat / 1000);
            }

            if parsed.nx.is_some() {
                let existing_data = db.get(key.clone());

                if existing_data.is_none() {
                    db.set(key.clone(), DataTypes::String(data.to_string()));
                    return serializer::str("OK");
                } else {
                    return serializer::nil();
                }
            } else if parsed.xx.is_some() {
                let existing_data = db.get(key.clone());

                if existing_data.is_some() {
                    db.set(key.clone(), DataTypes::String(data.to_string()));
                    return serializer::str("OK");
                } else {
                    return serializer::nil();
                }
            } else {
                db.set(key.clone(), DataTypes::String(data.to_string()));
                serializer::str("OK")
            }
        }

        Err(msg) => {
            let err_msg = format!("ERR syntax error. {}", msg.0);
            return serializer::err(&err_msg);
        }
    }
}

fn handle_get(query: &Query) -> Vec<u8> {
    let db = DatabaseOps;
    let key = query.command_str.to_string();

    let data = db.get(key);

    let response = match data {
        Some(data) => match data {
            DataTypes::String(data) => serializer::bulk_str(&data),
        },
        None => serializer::nil(),
    };

    response
}

fn handle_keys(query: &Query) -> Vec<u8> {
    let db = DatabaseOps;
    let keys = db.keys();
    let pattern = Pattern::new(&query.command_str).unwrap();

    if query.command_str.is_empty() {
        return serializer::err("ERR wrong number of arguments for 'keys' command");
    }

    if keys.len() == 0 {
        return serializer::nil();
    }

    if query.command_str.is_empty() {
        return serializer::str_arr(&keys);
    }

    let keys: Vec<String> = keys
        .iter()
        .filter(|key| pattern.matches(key))
        .map(|key| key.to_string())
        .collect();

    serializer::str_arr(&keys)
}

fn handle_delete(query: &Query) -> Vec<u8> {
    let db = DatabaseOps;
    let key = &query.command_str;
    let result = db.del(key.to_string());

    match result {
        Some(_) => serializer::int(1),
        None => serializer::int(0),
    }
}

fn handle_exists(query: &Query) -> Vec<u8> {
    let db = DatabaseOps;
    let mut count = 0;
    let first_key = &query.command_str; // fist key is the command itself
    let mut other_keys = query.args.as_slice().to_vec();

    other_keys.push(first_key.to_string());

    for key in other_keys {
        let result = db.get(key);

        if result.is_some() {
            count += 1;
        }
    }

    serializer::int(count)
}

#[derive(Debug)]
enum IncrDecrOpts {
    Incr,
    Decr,
}

fn handle_incr_decr(query: &Query, ops: IncrDecrOpts) -> Vec<u8> {
    let mut db = DatabaseOps;
    let key = &query.command_str;
    let data = db.get(key.to_string());

    println!("data is {:?}", ops);

    match data {
        Some(data) => match data {
            DataTypes::String(data) => {
                let data = data.parse::<i64>();

                if data.is_err() {
                    return serializer::err("ERR value is not an integer or out of range");
                }

                let data = data.unwrap();

                let result = match ops {
                    IncrDecrOpts::Incr => data + 1,
                    IncrDecrOpts::Decr => data - 1,
                };

                db.set(key.to_string(), DataTypes::String(result.to_string()));
                return serializer::int(result);
            }
        },
        None => {
            let data_to_store = match ops {
                IncrDecrOpts::Incr => 1,
                IncrDecrOpts::Decr => -1,
            };

            db.set(
                key.to_string(),
                DataTypes::String(data_to_store.to_string()),
            );

            return serializer::int(data_to_store);
        }
    };
}
