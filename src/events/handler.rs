use std::net::SocketAddr;

use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::TryStreamExt, SinkExt, StreamExt};

use crate::{
    helpers::function::user::{add_addr_in_users, get_all_other_u_sender, remove_user},
    models::user::User,
};
use tokio::net::TcpStream;

pub async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (tx, mut rx) = unbounded();
    let current_user = User::new(addr.clone());
    add_addr_in_users(current_user.clone(), tx);

    let (mut ws_writer, mut ws_read) = ws_stream.split();
    loop {
        tokio::select! {
          Some(msg)=ws_read.next()=>{
            match msg {
              Ok(msg) => {
                println!("Received a message from {}: {}", addr, msg.to_text().unwrap());

                for recp in get_all_other_u_sender(current_user) {
                    recp.unbounded_send(msg.clone()).unwrap();
                }
              }
              Err(e) => {
                println!("Error reading message from {}: {:?}", addr, e);
                break;
              }
            }
          }
          Some(m)=rx.next() =>{
            ws_writer.send(m).await.expect("Failed to send msg");
          }
        }
    }
}
