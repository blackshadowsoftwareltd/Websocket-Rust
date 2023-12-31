use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::sync::mpsc::unbounded_channel;
use tokio_tungstenite::tungstenite::{
    handshake::server::{Request, Response},
    Message,
};

use crate::{
    events::sender::{
        connection::{get_existing_connections, new_connection_notify},
        message::{send_message, send_message_to_all_other_users},
    },
    helpers::function::{
        socket_addr::add_socket_addr,
        validation::validate_message_type,
        ws::{ws_disconnected, ws_header_validation},
    },
};
use tokio::net::TcpStream;

pub async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) -> Result<()> {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream =
        tokio_tungstenite::accept_hdr_async(raw_stream, |req: &Request, res: Response| {
            ws_header_validation(req, res)
        })
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, mut rx) = unbounded_channel::<Message>();
    add_socket_addr(addr.clone(), tx.clone());

    let (mut ws_writer, mut ws_read) = ws_stream.split();

    get_existing_connections(tx.clone(), addr.clone())?;
    new_connection_notify(addr.clone())?;
    loop {
        tokio::select! {
          Some(msg)=ws_read.next()=>{
            match msg {
              Ok(msg) => {
                match ws_disconnected(addr.clone(),msg.clone())?{
                  true=>{ break; }
                  false=>{
                    println!("Received a message from {}: {}", addr, msg.to_text()?);
                  let (message,raw)=  validate_message_type(msg.clone())?;
                    if raw.is_some(){
                      send_message_to_all_other_users(addr.clone(),raw.unwrap())?;
                    } else if message.is_some(){
                      send_message(addr.clone(),tx.clone(),message.unwrap())?;
                    }
                  }
                }
              }
              Err(e) => {
                println!("Error reading message from {}: {:?}", addr, e);
                break
              }
            }
          }
          Some(m)=rx.recv() =>{
            ws_writer.send(m).await.expect("Failed to send msg");
          }
        }
    }
    println!("Client {} disconnected", addr);
    anyhow::Ok(())
}
