mod resp_serde;

use resp_serde::de;
use resp_serde::ser;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

static IP: &str = "0.0.0.0:6379";

fn main() {
    let listener = TcpListener::bind(IP).unwrap();

    println!("Server listening on {}", IP);

    for stream in listener.incoming() {
        println!("New client!");

        match stream {
            Ok(stream) => {
                spawn(move || {
                    handle_commands(&stream);
                });
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_commands(mut stream: &TcpStream) {
    let mut buffer = [0; 1024 * 10]; // 10KB buffer

    loop {
        match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                let data = &buffer[..size];
                let response = String::from_utf8_lossy(data);
                let query = de::Query::new(&response);

                match query.command {
                    de::Command::Ping => {
                        stream
                            .write_all(&ser::str("Hello from the Other Side"))
                            .expect("Failed to write to stream");
                    }

                    _ => {
                        stream
                            .write_all(&ser::err("Command not implemented"))
                            .expect("Failed to write to stream");
                    }
                }
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
