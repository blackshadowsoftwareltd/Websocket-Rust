# Rust WebSocket Example with tokio-tungstenite

This is a simple example project demonstrating WebSocket implementation in Rust using the `tokio-tungstenite` crate. The project provides a basic WebSocket server written in Rust.

## Features

- WebSocket server implementation using `tokio-tungstenite`.
- Simple communication.

## Requirements

- Rust: Make sure you have Rust installed. If not, you can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Usage

### 1. Clone the Repository

```bash
git clone https://github.com/blackshadowsoftwareltd/Websocket-Rust.git
cd Websocket-Rust
```

### 2. Build and Run the Server

```bash
cargo run --release
```

#### After starting the server, you can see the IP address. now you can connect your Postman with this IP.

### 3. Headers.

### Before connecting your connection, you have to give the Header.

```bash
key : Authorization
value : token
```

#### Connect your multiple clients to send messages to other connections. after establishing a new connection you will see the IP of this connection.

### 4. To send a specific connection use this payload

```bash
{
    "to":"127.0.0.1:48466",
    "text":"Hello!"
}
```

### Note :

1. If you do not have a provider ID, the message will be sent to all connections.
2. If you do not maintain this format (payload). or use any kind of String, the message will be sent to all of the connections.
