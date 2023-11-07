use futures_channel::mpsc::UnboundedSender;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use tokio_tungstenite::tungstenite::protocol::Message;

pub type WSState = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
