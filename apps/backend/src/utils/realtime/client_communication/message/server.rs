use serde::Serialize;

use crate::models::r#match::{JoinResponse, MatchState};

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    JoinResponse(JoinResponse),
    MoveResult(Option<MatchState>),
    NewState(MatchState),
}
