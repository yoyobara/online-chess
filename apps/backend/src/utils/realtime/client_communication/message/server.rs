use rust_chess::core::chess_move::Move;
use serde::Serialize;

use crate::models::{chat::ChatMessage, r#match::{JoinResponse, MatchState}};

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    JoinResponse(JoinResponse),
    MoveResult(bool),
    PlayerMove(Move, MatchState),
    ChatMessage(ChatMessage),
}
