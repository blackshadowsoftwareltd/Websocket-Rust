use std::{net::SocketAddr, str::FromStr};

use anyhow::Result;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    helpers::function::socket_addr::{get_all_other_u_sender, get_u_sender},
    models::message::TextMessage,
};

pub fn send_message(
    addr: SocketAddr,
    sender: UnboundedSender<Message>,
    message: TextMessage,
) -> Result<()> {
    if message.to.is_none() {
        send_message_to_all_other_users(addr.clone(), message.text.unwrap())?;
        return Ok(());
    }
    let receiver = SocketAddr::from_str(&message.to.unwrap())?;
    match get_u_sender(receiver) {
        Some(recp) => {
            recp.send(Message::Text(message.text.unwrap()))?;
        }
        None => {
            sender.send(Message::Text(format!(
                "User not found: {}",
                receiver.to_string()
            )))?;
        }
    }
    Ok(())
}

pub fn send_message_to_all_other_users(addr: SocketAddr, msg: String) -> Result<()> {
    let msg = Message::Text(msg);
    for recp in get_all_other_u_sender(addr.clone()) {
        recp.send(msg.clone())?;
    }
    Ok(())
}
