use tokio_tungstenite::tungstenite::Message;

pub async fn handle_ws_read(msg: Message) {
    println!("message received from ws : {:?}", msg.to_text())
}
