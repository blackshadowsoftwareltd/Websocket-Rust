use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub async fn handle_channel_events(
    mut ws_receiver: UnboundedReceiver<Vec<u8>>,
    provider_sender: UnboundedSender<Vec<u8>>,
) {
    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(msg)=ws_receiver.recv()=>{
                    provider_sender.send(msg).expect("Failed to send Provder")
                }
            }
        }
    });
}
