use serde::Serialize;

use crate::models::r#match::JoinResponse;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    JoinResponse(JoinResponse),
    // MatchState(MatchState),
}
