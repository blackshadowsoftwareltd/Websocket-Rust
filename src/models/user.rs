use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub name: Option<String>,
    pub id: SocketAddr,
}

impl User {
    pub fn new(id: SocketAddr) -> Self {
        Self { name: None, id }
    }
}
