use rust_chess::{board::Board, core::color::Color};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MatchState {
    pub board: Board,
    pub move_count: i32,
}

#[derive(Serialize, Debug)]
pub struct JoinResponse {
    pub initial_state: MatchState,
    pub color: Color,
    pub opponent_id: i32,
}

#[derive(Serialize, Debug)]
pub struct MatchPlayers {
    pub white_player_id: i32,
    pub black_player_id: i32,
}
