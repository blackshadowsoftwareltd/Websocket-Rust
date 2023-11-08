use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};

use crate::models::user::User;

pub static USERS: OnceLock<Mutex<HashSet<User>>> = OnceLock::new();
pub static CURRENT_USER: OnceLock<Mutex<HashSet<User>>> = OnceLock::new();
