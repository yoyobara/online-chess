use serde::{Deserialize, Serialize};

use crate::models::r#match::MatchState;

#[derive(Debug, Serialize, Deserialize)]
pub enum PubSubMessage {
    MatchmakingMatchId(String),
    PlayerMove(MatchState),
}
