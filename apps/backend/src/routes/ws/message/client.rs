use serde::Deserialize;

#[derive(PartialEq, Eq, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientMessage {
    LookingForMatch,
}
