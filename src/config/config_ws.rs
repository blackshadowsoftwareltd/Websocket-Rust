use std::{collections::HashMap, sync::Mutex};

use tokio::net::TcpListener;

use crate::helpers::{
    types::{PeerMap, WSState},
    ws_address::ws_ip_address,
};
use anyhow::{Ok, Result};
pub async fn ws_config() -> Result<(WSState, TcpListener)> {
    let addr = ws_ip_address().await;
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket?;
    println!("Listening on: {}", addr);
    Ok((state, listener))
}
