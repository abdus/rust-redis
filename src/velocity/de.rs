use super::command::Command;

#[derive(Debug)]
pub struct Query {
    pub value: String,
    pub command: Command,
    pub args: Option<Vec<String>>,
}

impl Query {
    pub fn new(query: &str) -> Query {
        let mut query = query.split("\r\n");
        let data_type = query.next(); // *len

        match data_type {
            Some(data) => {
                if !data.starts_with("*") {
                    return Query {
                        value: "query is not an array".to_string(),
                        command: Command::Unknown,
                        args: None,
                    };
                }
            }

            None => {
                return Query {
                    value: "query is not an array".to_string(),
                    command: Command::Unknown,
                    args: None,
                };
            }
        }

        // remove argument length from RESP2 query
        let _ = query.next().unwrap(); // $len
        let command_str = query.next();

        let command = match command_str {
            Some(command_str) => Command::from_str(command_str),
            None => {
                return Query {
                    value: "query is not an array".to_string(),
                    command: Command::Unknown,
                    args: None,
                };
            }
        };

        // remove argument length from RESP2 query
        let _ = query.next(); // $len

        let value = match query.next() {
            Some(value) => value,
            None => "",
        };

        let mut args: Vec<String> = vec![];

        for (_, arg) in query.enumerate() {
            // we do not need length of the query
            if arg.starts_with("$") {
                continue;
            }

            args.push(arg.to_string());
        }

        let query = Query {
            value: value.to_string(),
            command,
            args: Some(args),
        };

        println!("command: {:?}", query);

        query
    }

    pub fn create_response(&self) -> Vec<u8> {
        let command = &self.command;
        command.create_response(self)
    }
}
