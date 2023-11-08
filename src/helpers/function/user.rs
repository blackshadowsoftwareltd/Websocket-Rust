use std::{collections::HashSet, net::SocketAddr, sync::Mutex};

use crate::{
    helpers::oncelock::users::{CURRENT_USER, USERS},
    models::user::User,
};

// ---------- Users ----------
pub fn init_users() {
    let mut set = USERS
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    set.clear();
}

pub fn add_user(user: User) {
    let mut set = USERS
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    set.insert(user);
}

pub fn get_user(user: &User) -> Option<User> {
    let set = USERS
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    set.get(user).cloned()
}

pub fn remove_user(user: User) {
    let mut set = USERS
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    set.remove(&user);
}

pub fn get_all_users() -> Vec<User> {
    let set = USERS
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    set.iter().cloned().collect()
}

pub fn get_all_other_users(user: User) -> Vec<User> {
    let set = USERS
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    set.clone().into_iter().filter(|x| x != &user).collect()
}

pub fn add_addr_in_users(addr: SocketAddr) {
    {
        let user = User::new(addr.clone());
        add_user(user);
    }
    for x in get_all_users().iter() {
        println!("user: {:?}", x);
    }
}

// ---------- Current User ----------
pub fn init_current_user() {
    let mut set = CURRENT_USER
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    set.clear();
}

pub fn set_current_user(user: User) {
    let mut set = USERS
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    {
        set.clear();
    }
    {
        set.insert(user);
    }
}

pub fn get_current_user() -> Option<User> {
    let set = USERS
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    set.iter().cloned().last()
}
