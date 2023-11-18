use crate::models::message::TextMessage;
use anyhow::Result;
use serde_json::{from_str, from_value, Value};
use tokio_tungstenite::tungstenite::Message;

pub fn validate_message_type(msg: Message) -> Result<(Option<TextMessage>, Option<String>)> {
    let msg = msg.to_string();
    let parsed: Result<Value, _> = from_str(&msg.clone());
    match parsed {
        Ok(value) => {
            let r: Result<TextMessage, _> = from_value(value);
            match r {
                Ok(message) => Ok((Some(message), None)),
                Err(_) => Ok((None, Some(msg))),
            }
        }
        Err(_) => Ok((None, Some(msg))),
    }
}
