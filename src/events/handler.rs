use futures_util::{stream::SplitSink, SinkExt};
use tokio::{net::TcpStream, sync::mpsc::UnboundedSender};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

pub async fn handle_ws_read(msg: Message, ws_sender: UnboundedSender<Vec<u8>>) {
    let msg = msg.to_text().unwrap();
    ws_sender
        .send(msg.as_bytes().to_vec())
        .expect("Failed to send via channel");
}

pub async fn handle_ws_write(
    ws_write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    msg: Vec<u8>,
) {
    let msg = String::from_utf8(msg).expect("Failed to decode");
    let msg = Message::Text(msg);
    ws_write.send(msg).await.expect("Failed to send msg");
}
