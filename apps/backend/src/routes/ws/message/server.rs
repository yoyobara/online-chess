use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    MatchFound,
}
