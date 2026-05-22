use rust_chess::core::{
    chess_move::{Move, MoveType},
    piece::PieceType,
    square::Square,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerMoveData {
    pub src_square: Square,
    pub dest_square: Square,
    pub promotion: Option<PieceType>,
    pub move_type: MoveType,
}

impl From<PlayerMoveData> for Move {
    fn from(value: PlayerMoveData) -> Self {
        Self {
            from: value.src_square,
            to: value.dest_square,
            move_type: value.move_type,
            promotion: value.promotion,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    JoinGame,
    PlayerMove(PlayerMoveData),
}
