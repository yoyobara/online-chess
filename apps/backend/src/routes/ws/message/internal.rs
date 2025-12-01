use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum InternalMessage {
    MatchFound,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InternalMessageWithReciever {
    pub to_user: i32,
    pub message: InternalMessage,
}
