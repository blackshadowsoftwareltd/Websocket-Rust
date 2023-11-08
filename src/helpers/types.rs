use futures_channel::mpsc::UnboundedSender;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio_tungstenite::tungstenite::protocol::Message;

use crate::models::user::User;

pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<User, Tx>>>;
