use futures_channel::mpsc::UnboundedSender;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

use crate::{helpers::port::free_port, models::socket::WSData};
use std::{
    collections::HashMap,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

pub async fn ws_connect() -> Result<WSData, IoError> {
    let addr = free_port().await.unwrap();

    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let try_socket: Result<TcpListener, IoError> = TcpListener::bind(&addr).await;
    println!("WS connecting to : {}", addr);
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on WS : {}", addr);

    Ok(WSData {
        state,
        listener,
        addr,
    })
}
