use anyhow::{Ok, Result};

use crate::{
    helpers::{
        extension::{user::UserInfoExt, ws_message::WsMessageExt, ws_msg_type::WsMsgTypeExt},
        function::user::{get_all_other_u_sender, get_all_u_sender, remove_user},
    },
    models::user::{User, UserInfo},
};

pub fn new_connection_notify(c: User) -> Result<()> {
    for recp in get_all_other_u_sender(c) {
        let msg = UserInfo::new("Mr".to_string(), c.addr.to_string())
            .to_new_conn()
            .to_json()?
            .to_ws_msg_text();
        recp.send(msg)?;
    }
    Ok(())
}

pub fn close_connection_notify(c: User) -> Result<()> {
    remove_user(c);
    for recp in get_all_u_sender() {
        let msg = UserInfo::new("Mr".to_string(), c.addr.to_string())
            .to_dis_conn()
            .to_json()?
            .to_ws_msg_text();
        recp.send(msg)?;
    }
    Ok(())
}
