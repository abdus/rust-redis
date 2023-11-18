#[derive(Debug)]
pub enum Command {
    Ping,
    Get,
    Unknown,
}

#[derive(Debug)]
pub struct Query {
    pub value: String,
    pub command: Command,
}

impl Query {
    pub fn new(query: &str) -> Query {
        let mut query = query.split_whitespace();

        let data_type = query.next(); // *len

        match data_type {
            Some(data) => {
                if !data.starts_with("*") {
                    return Query {
                        value: "query is not an array".to_string(),
                        command: Command::Unknown,
                    };
                }
            }

            None => {
                return Query {
                    value: "query is not an array".to_string(),
                    command: Command::Unknown,
                };
            }
        }

        // remove argument length from RESP2 query
        let _ = query.next().unwrap(); // $len

        let command = match query.next().unwrap().to_lowercase().as_str() {
            "get" => Command::Get,
            "ping" => Command::Ping,
            c => {
                println!("Unknown command: {}", c);
                Command::Unknown
            }
        };

        // remove argument length from RESP2 query
        let _ = query.next(); // $len

        let value = match query.next() {
            Some(value) => value,
            None => "",
        };

        let query = Query {
            value: value.to_string(),
            command,
        };

        query
    }
}
