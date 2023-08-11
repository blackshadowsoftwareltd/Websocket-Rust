use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::UnboundedSender;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

pub struct WSData {
    pub state: Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>,
    pub listener: TcpListener,
    pub addr: String,
}
