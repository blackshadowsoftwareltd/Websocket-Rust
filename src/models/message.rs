use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextMessage {
    pub text: Option<String>,
    pub to: Option<String>,
}
