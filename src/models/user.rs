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
