use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};

use crate::{helpers::types::SocketHashMap, models::user::User};

// pub static USERS: UsersHashMap = OnceLock::new();
pub static SOCKET_ADDRS: SocketHashMap = OnceLock::new();
pub static CURRENT_USER: OnceLock<Mutex<HashSet<User>>> = OnceLock::new();
