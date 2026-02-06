use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerMoveData {
    src_square: String,
    dest_sqaure: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    JoinGame,
    PlayerMove(PlayerMoveData),
}
