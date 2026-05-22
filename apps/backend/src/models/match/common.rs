use rust_chess::{board::Board, core::color::Color};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum MatchResult {
    Win(Color),
    Draw,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchState {
    pub board: Board,
    pub current_turn: Color,
    pub match_result: Option<MatchResult>,
}
