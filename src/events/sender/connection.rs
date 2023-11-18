use std::net::SocketAddr;

use anyhow::{Ok, Result};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    helpers::{
        extension::{
            user::{UserInfoExt, UserInfoListExt},
            ws_message::WsMessageExt,
            ws_msg_type::WsMsgTypeExt,
        },
        function::socket_addr::{
            get_all_other_socket_addrs, get_all_other_u_sender, get_all_u_sender,
            remove_socket_addr,
        },
    },
    models::user::UserInfo,
};

pub fn new_connection_notify(addr: SocketAddr) -> Result<()> {
    for recp in get_all_other_u_sender(addr) {
        let msg = UserInfo::new("Mr".to_string(), addr.to_string())
            .to_new_conn()
            .to_json()?
            .to_ws_msg_text();
        recp.send(msg)?;
    }
    Ok(())
}

pub fn close_connection_notify(addr: SocketAddr) -> Result<()> {
    remove_socket_addr(addr);
    for recp in get_all_u_sender() {
        let msg = UserInfo::new("Mr".to_string(), addr.to_string())
            .to_dis_conn()
            .to_json()?
            .to_ws_msg_text();
        recp.send(msg)?;
    }
    Ok(())
}
pub fn get_existing_connections(tx: UnboundedSender<Message>, addr: SocketAddr) -> Result<()> {
    let mut others: Vec<UserInfo> = vec![];
    for addr in get_all_other_socket_addrs(addr) {
        others.push(UserInfo::new("Mr".to_string(), addr.to_string()));
    }
    let msg = others.to_new_conn().to_json()?.to_ws_msg_text();
    tx.send(msg)?;
    Ok(())
}
