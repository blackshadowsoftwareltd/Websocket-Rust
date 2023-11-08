use futures_channel::mpsc::UnboundedSender;
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use tokio_tungstenite::tungstenite::protocol::Message;

use crate::models::user::User;

pub type USender = UnboundedSender<Message>;
pub type UsersHashMap = OnceLock<Mutex<HashMap<User, USender>>>;
