use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Mutex, OnceLock},
};
use tokio::sync::mpsc::UnboundedSender;

use tokio_tungstenite::tungstenite::protocol::Message;

pub type USender = UnboundedSender<Message>;
pub type SocketHashMap = OnceLock<Mutex<HashMap<SocketAddr, USender>>>;
