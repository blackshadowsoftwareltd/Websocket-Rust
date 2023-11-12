use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub name: Option<String>,
    pub addr: SocketAddr,
}

impl User {
    pub fn new(addr: SocketAddr) -> Self {
        Self { name: None, addr }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserInfo {
    pub name: Option<String>,
    pub ws_id: Option<String>,
}

impl UserInfo {
    pub fn new(n: String, w: String) -> Self {
        UserInfo {
            name: Some(n),
            ws_id: Some(w),
        }
    }
}
