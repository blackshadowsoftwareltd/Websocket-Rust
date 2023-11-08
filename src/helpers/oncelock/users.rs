use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};

use crate::{helpers::types::UsersHashMap, models::user::User};

pub static USERS: UsersHashMap = OnceLock::new();
pub static CURRENT_USER: OnceLock<Mutex<HashSet<User>>> = OnceLock::new();
