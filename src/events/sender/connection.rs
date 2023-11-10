use anyhow::{Ok, Result};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    helpers::{
        enums::ws_msg_type::WsMsgType,
        function::user::{get_all_other_u_sender, get_all_u_sender, remove_user},
    },
    models::user::{User, UserInfo},
};

pub fn new_connection_notify(c: User) -> Result<()> {
    for recp in get_all_other_u_sender(c) {
        let info = UserInfo::new("Mr".to_string(), c.addr.to_string());
        let info = WsMsgType::NewConn(info);
        let json = serde_json::to_string(&info)?;
        let msg = Message::Text(json);
        recp.send(msg)?;
    }
    Ok(())
}

pub fn close_connection_notify(c: User) -> Result<()> {
    remove_user(c);
    for recp in get_all_u_sender() {
        let info = UserInfo::new("Mr".to_string(), c.addr.to_string());
        let info = WsMsgType::DisConn(info);
        let json = serde_json::to_string(&info)?;
        let msg = Message::Text(json);
        recp.send(msg)?;
    }
    Ok(())
}
