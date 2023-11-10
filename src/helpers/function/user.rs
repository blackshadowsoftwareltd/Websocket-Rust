use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use crate::{
    helpers::{
        oncelock::users::{CURRENT_USER, USERS},
        types::USender,
    },
    models::user::User,
};

// ---------- Users ----------
pub fn init_users() {
    let mut set = USERS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.clear();
}

pub fn add_user(user: User, u_sender: USender) {
    let mut set = USERS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.insert(user, u_sender);
}

pub fn get_u_sender(user: &User) -> Option<USender> {
    let set = USERS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.get(user).cloned()
}

pub fn remove_user(user: User) {
    let mut set = USERS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.remove(&user);
}

pub fn get_all_users() -> Vec<User> {
    let set = USERS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.iter().map(|(u, _)| u).cloned().collect()
}

pub fn get_all_other_users(user: User) -> Vec<User> {
    let set = USERS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.clone()
        .into_iter()
        .map(|(u, _)| u)
        .filter(|x| x != &user)
        .collect()
}

pub fn get_all_other_u_sender(user: User) -> Vec<USender> {
    let set = USERS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    set.clone()
        .into_iter()
        .filter(|(u, _)| u != &user)
        .map(|(_, s)| s)
        .collect()
}

pub fn get_all_u_sender() -> Vec<USender> {
    let set = USERS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    set.clone().into_iter().map(|(_, s)| s).collect()
}

pub fn add_addr_in_users(u: User, u_sender: USender) {
    {
        add_user(u, u_sender);
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
    let mut set = CURRENT_USER
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
    let set = CURRENT_USER
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap();
    set.iter().cloned().last()
}
