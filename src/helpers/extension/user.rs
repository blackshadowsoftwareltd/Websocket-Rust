use tokio_tungstenite::tungstenite::Message;

use crate::models::user::User;
pub trait UserExt {
    fn to_ws_user(&self) -> Message;
}
pub trait UsersExt {
    fn to_ws_users(&self) -> Message;
}

impl UserExt for User {
    fn to_ws_user(&self) -> Message {
        let users = serde_json::to_string(&self).unwrap();
        Message::Text(users)
    }
}

impl UsersExt for Vec<User> {
    fn to_ws_users(&self) -> Message {
        let users = serde_json::to_string(&self).unwrap();
        Message::Text(users)
    }
}
