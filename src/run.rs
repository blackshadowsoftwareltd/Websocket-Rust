use crate::{connect::ws_connect, events::event::ws_events};

pub async fn run_ws() {
    let ws_data = ws_connect().await.expect("!!! WS connecting Error");
    ws_events(ws_data).await;
    loop {}
}
