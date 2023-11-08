use tokio::net::TcpListener;

use crate::helpers::ws_address::ws_ip_address;
use anyhow::{Ok, Result};
pub async fn ws_config() -> Result<TcpListener> {
    let addr = ws_ip_address().await;
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket?;
    println!("Listening on: {}", addr);
    Ok(listener)
}
