use rust_chess::core::square::Square;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerMoveData {
    pub src_square: Square,
    pub dest_square: Square,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    JoinGame,
    PlayerMove(PlayerMoveData),
}
