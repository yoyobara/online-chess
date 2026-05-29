use rust_chess::core::{chess_move::Move, color::Color};
use serde::Serialize;

use crate::models::r#match::MatchState;

#[derive(Serialize, Debug)]
pub struct JoinResponse {
    pub initial_state: MatchState,
    pub color: Color,
    pub opponent_id: i32,
    pub initial_moves: Vec<Move>,
}

#[derive(Serialize, Debug)]
pub struct MatchPlayers {
    pub white_player_id: i32,
    pub black_player_id: i32,
}
