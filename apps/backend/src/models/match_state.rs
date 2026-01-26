use rust_chess::board::Board;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MatchState {
    pub board: Board,
    pub move_count: i32,
}
