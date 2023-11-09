use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Copy, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub name: Option<u8>,
    pub addr: SocketAddr,
}

impl User {
    pub fn new(addr: SocketAddr) -> Self {
        Self { name: None, addr }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NewUser {
    pub name: Option<String>,
    pub ws_id: Option<String>,
}

impl NewUser {
    pub fn new(n: String, w: String) -> Self {
        NewUser {
            name: Some(n),
            ws_id: Some(w),
        }
    }
}
