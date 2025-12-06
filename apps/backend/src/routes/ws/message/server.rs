use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Debug)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Log { message: &'static str },
}
