use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientMessage {
    LookingForMatch,
}
