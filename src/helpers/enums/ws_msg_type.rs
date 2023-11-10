use serde::{Deserialize, Serialize};

use crate::models::user::UserInfo;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WsMsgType {
    NewConn(UserInfo),
    DisConn(UserInfo),
}
