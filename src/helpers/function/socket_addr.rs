use std::{collections::HashMap, net::SocketAddr, sync::Mutex};

use crate::helpers::{oncelock::users::SOCKET_ADDRS, types::USender};

// ---------- Users ----------
pub fn init_socket_addrs() {
    let mut set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.clear();
}

pub fn add_socket_addr(addr: SocketAddr, u_sender: USender) {
    let mut set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.insert(addr, u_sender);
}

pub fn get_u_sender(addr: SocketAddr) -> Option<USender> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.get(&addr).cloned()
}

pub fn remove_socket_addr(addr: SocketAddr) {
    let mut set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.remove(&addr);
}

pub fn get_all_socket_addrs() -> Vec<SocketAddr> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.iter().map(|(u, _)| u).cloned().collect()
}

pub fn get_all_other_socket_addrs(addr: SocketAddr) -> Vec<SocketAddr> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();
    set.clone()
        .into_iter()
        .map(|(u, _)| u)
        .filter(|x| x != &addr)
        .collect()
}

pub fn get_all_other_u_sender(addr: SocketAddr) -> Vec<USender> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    set.clone()
        .into_iter()
        .filter(|(u, _)| u != &addr)
        .map(|(_, s)| s)
        .collect()
}

pub fn get_all_u_sender() -> Vec<USender> {
    let set = SOCKET_ADDRS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap();

    set.clone().into_iter().map(|(_, s)| s).collect()
}
