use anyhow::{Ok, Result};
use tokio_tungstenite::tungstenite::Message;

use crate::{events::sender::connection::close_connection_notify, models::user::User};

pub fn ws_disconnected(c: User, msg: Message) -> Result<bool> {
    if msg.is_close() && msg.is_empty() {
        close_connection_notify(c)?;
        return Ok(true);
    }
    return Ok(false);
}
