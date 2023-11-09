use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};
use tokio::sync::mpsc::UnboundedSender;

use tokio_tungstenite::tungstenite::protocol::Message;

use crate::models::user::User;

pub type USender = UnboundedSender<Message>;
pub type UsersHashMap = OnceLock<Mutex<HashMap<User, USender>>>;
