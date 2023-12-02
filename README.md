# Redis Implementation

This Rust project implements a simple RESP2 (REdis Serialization Protocol 2)
server. RESP2 is a binary-safe protocol used by Redis for communication between
clients and servers. The server listens on the specified IP address and port
(0.0.0.0:6379 by default) and handles basic RESP2 commands such as PING and
GET.

## Table of Contents

- [Getting Started](#getting-started)
- [Usage](#usage)
- [File Structure](#file-structure)
- [Contributing](#contributing)
- [License](#license)

## Getting Started

To get started with the RESP2 server, ensure you have Rust installed on your
system. You can install Rust by following the instructions on the [official
Rust website](https://www.rust-lang.org/).

Clone the repository:

```bash
git clone https://github.com/abdus/redis-server.git
cd redis-server
```

Build and run the server:

```rust
cargo run
```

By default, the server will listen on 0.0.0.0:6379. You can modify the IP
constant in the main.rs file to change the IP address and port.

## Usage

Connect to the RESP2 server using a Redis client or any RESP2-compatible tool.
The server currently supports the PING command. For any other command, it will
respond with an error indicating that the command is not implemented.

Example using the redis-cli tool:

```bash
$ redis-cli -p 6379
127.0.0.1:6379>PING
Hello from the Other Side
```
