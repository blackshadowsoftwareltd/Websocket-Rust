use crate::{
    connect::ws_connect,
    events::{event::ws_events, handle_chalnnel::handle_channel_events},
};

pub async fn run_ws() {
    let ws_data = ws_connect().await.expect("!!! WS connecting Error");

    let (ws_sender, ws_receiver) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
    let (provider_sender, provider_receiver) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();

    ws_events(ws_data, ws_sender, provider_receiver).await;
    handle_channel_events(ws_receiver, provider_sender).await;
    loop {}
}
