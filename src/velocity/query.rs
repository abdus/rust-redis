use super::command::Command;

#[derive(Debug)]
pub struct Query {
    pub command_str: String,
    pub command_enum: Command,
    pub args: Vec<String>,
}

impl Query {
    pub fn new(query: &str) -> Query {
        /*
         * Query Example
         * -------------
         * *4
         * $3
         * SET
         * $1
         * k
         * $1
         * v
         * $2
         * NX
         */

        let mut query = query.split("\r\n");
        let data_type = query.next(); // *len

        match data_type {
            Some(data) => {
                if !data.starts_with("*") {
                    return Query {
                        command_str: "query is not an array".to_string(),
                        command_enum: Command::Unknown,
                        args: vec![],
                    };
                }
            }

            None => {
                return Query {
                    command_str: "query is not an array".to_string(),
                    command_enum: Command::Unknown,
                    args: vec![],
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
                    command_str: "query is not an array".to_string(),
                    command_enum: Command::Unknown,
                    args: vec![],
                };
            }
        };

        // remove argument length from RESP2 query
        let _ = query.next(); // $len

        let value = match query.next() {
            Some(value) => value,
            None => "",
        };

        // the command arguments are the rest of the query
        let mut args: Vec<String> = vec![];

        for (_, arg) in query.enumerate() {
            // we do not need length of the query
            if arg.starts_with("$") || arg.is_empty() {
                continue;
            }

            args.push(arg.to_string());
        }

        let query = Query {
            command_str: value.to_string(),
            command_enum: command,
            args,
        };

        query
    }

    pub fn create_response(&self) -> Vec<u8> {
        let command = &self.command_enum;
        command.create_response(self)
    }
}
