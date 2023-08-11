use futures_util::StreamExt;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    events::handler::{handle_ws_read, handle_ws_write},
    models::socket::WSData,
};

pub async fn ws_events(
    ws_data: WSData,
    ws_sender: UnboundedSender<Vec<u8>>,
    mut provider_receiver: UnboundedReceiver<Vec<u8>>,
) {
    tokio::spawn(async move {
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
                    handle_ws_read(msg.clone(), ws_sender.clone()).await;
                },
                Some(msg)=provider_receiver.recv()=>{
                    handle_ws_write(&mut ws_write,msg).await;
                }
            }
        }
    });
}
