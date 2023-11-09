use anyhow::{Ok, Result};
use tokio_tungstenite::tungstenite::Message;

use crate::{
    helpers::function::user::get_all_other_u_sender,
    models::user::{NewUser, User},
};

pub fn new_connection_notify(c: User) -> Result<()> {
    for recp in get_all_other_u_sender(c) {
        let info = NewUser::new("Mr".to_string(), c.addr.to_string());
        let json = serde_json::to_string(&info)?;
        let msg = Message::Text(json);
        recp.send(msg)?;
    }
    Ok(())
}
