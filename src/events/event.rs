use std::thread;

use futures_util::{SinkExt, StreamExt};

use crate::{events::handler::handle_ws_read, models::socket::WSData};

pub async fn ws_events(ws_data: WSData) {
    _ = thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let (raw_stream, _) = ws_data
                .listener
                .accept()
                .await
                .expect("Failed to accept listener");

            let stream = tokio_tungstenite::accept_async(raw_stream)
                .await
                .expect("Failed to accept stream");
            println!("WebSocket connection established: {}", ws_data.addr);
            let (mut ws_write, mut ws_read) = stream.split();

            loop {
                tokio::select! {
                    Some(msg)=ws_read.next()=>{
                        let msg = msg.expect("Failed to get msg");
                        handle_ws_read(msg.clone()).await;
                        ws_write.send(msg).await.expect("Failed to send msg");
                    }
                }
            }
        });
    });
}
