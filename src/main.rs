mod velocity;

use crate::velocity::database::DatabaseOps;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use std::vec;
use velocity::query;

static IP: &str = "0.0.0.0:6379";

fn main() {
    let listener = TcpListener::bind(IP).expect("Failed to bind to port");

    println!("Server listening on {}", IP);

    for stream in listener.incoming() {
        spawn(move || match stream {
            Ok(stream) => {
                handle_commands(&stream);
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        });
    }

    println!("Server shutting down");
}

fn handle_commands(mut stream: &TcpStream) {
    let mut buffer = vec![0; 1024 * 100]; // 100kb buffer

    loop {
        match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                let data = &buffer[..size];
                let response = String::from_utf8_lossy(data);
                let query = query::Query::new(&response);
                let response = query.create_response();

                stream
                    .write_all(&response)
                    .expect("Failed to write to stream");
            }

            Ok(_) => {
                println!("Client closed the connection");
                break;
            }

            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}
