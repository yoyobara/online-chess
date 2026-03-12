use rust_chess::core::{chess_move::MoveType, piece::PieceType, square::Square};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerMoveData {
    pub src_square: Square,
    pub dest_square: Square,
    pub promotion: Option<PieceType>,
    pub move_type: MoveType,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    JoinGame,
    PlayerMove(PlayerMoveData),
}
