use crate::{routes::ws::message::ChessMessage, state::AppState};

pub async fn on_message(message: ChessMessage, player_id: i32, state: &AppState) {
    dbg!(message);
    dbg!(player_id);
    dbg!(state);
}
