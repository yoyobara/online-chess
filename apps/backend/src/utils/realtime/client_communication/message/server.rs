use serde::Serialize;

use crate::models::match_state::MatchState;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    MatchState(MatchState),
}
