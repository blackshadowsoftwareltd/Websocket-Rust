use std::net::SocketAddr;

pub async fn free_port() -> Option<String> {
    let start_port = 8080;
    let end_port = 8090;
    for port in start_port..=end_port {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        if std::net::TcpListener::bind(addr).is_ok() {
            return Some(addr.to_string());
        }
    }
    None
}
