use rust_chess::core::chess_move::Move;
use serde::{Deserialize, Serialize};

use crate::{
    models::{chat::ChatMessage, r#match::MatchState},
};

#[derive(Debug, Serialize, Deserialize)]
pub enum PubSubMessage {
    MatchmakingMatchId(String),
    PlayerMove(Move, MatchState),
    ChatMessage(ChatMessage),
}
