use tokio_tungstenite::tungstenite::Message;

pub trait WsMessageExt {
    fn to_ws_msg_text(&self) -> Message;
}

impl WsMessageExt for String {
    fn to_ws_msg_text(&self) -> Message {
        Message::Text(self.clone())
    }
}
