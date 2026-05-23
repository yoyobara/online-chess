use rust_chess::core::color::Color;
use serde::Serialize;

use crate::{
    models::r#match::MatchState, utils::realtime::client_communication::message::PlayerMoveData,
};

#[derive(Serialize, Debug)]
pub struct JoinResponse {
    pub initial_state: MatchState,
    pub color: Color,
    pub opponent_id: i32,
    pub initial_moves: Vec<PlayerMoveData>,
}

#[derive(Serialize, Debug)]
pub struct MatchPlayers {
    pub white_player_id: i32,
    pub black_player_id: i32,
}
