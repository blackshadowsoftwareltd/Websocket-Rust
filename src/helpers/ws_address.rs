use std::net::SocketAddr;

use async_recursion::async_recursion;
use tokio::net::TcpListener;

pub async fn ws_ip_address() -> String {
    SocketAddr::from(([127, 0, 0, 1], free_port().await)).to_string()
}

#[async_recursion]
async fn free_port() -> u16 {
    for port in 49152..=65535 {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        if let Ok(_) = TcpListener::bind(addr).await {
            return port;
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    free_port().await
}
