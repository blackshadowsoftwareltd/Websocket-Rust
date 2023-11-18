use tokio_tungstenite::tungstenite::Message;

use crate::{
    helpers::enums::ws_msg_type::WsMsgType,
    models::user::{User, UserInfo},
};
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

pub trait UserInfoExt {
    fn to_new_conn(&self) -> WsMsgType;
    fn to_dis_conn(&self) -> WsMsgType;
}

impl UserInfoExt for UserInfo {
    fn to_new_conn(&self) -> WsMsgType {
        WsMsgType::NewConn(self.clone())
    }

    fn to_dis_conn(&self) -> WsMsgType {
        WsMsgType::DisConn(self.clone())
    }
}
pub trait UserInfoListExt {
    fn to_new_conn(&self) -> WsMsgType;
}

impl UserInfoListExt for Vec<UserInfo> {
    fn to_new_conn(&self) -> WsMsgType {
        WsMsgType::ExistingConn(self.clone())
    }
}
