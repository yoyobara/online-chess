use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum PubSubMessage {
    MatchmakingMatchId(String),
}
