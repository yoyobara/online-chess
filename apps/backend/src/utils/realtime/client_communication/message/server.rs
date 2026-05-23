use serde::Serialize;

use crate::{
    models::r#match::{JoinResponse, MatchState},
    utils::realtime::client_communication::message::PlayerMoveData,
};

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    JoinResponse(JoinResponse),
    MoveResult(bool),
    PlayerMove(PlayerMoveData, MatchState),
}
