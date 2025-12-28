use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientMessage {
    LookingForMatch,
    ConnectionClosed,
}
