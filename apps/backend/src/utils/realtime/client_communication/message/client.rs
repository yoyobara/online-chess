use rust_chess::core::chess_move::Move;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    JoinGame,
    PlayerMove(Move),
    ChatMessage(String),
}
