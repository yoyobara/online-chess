use crate::{
    routes::ws::message::{ClientMessage, ServerMessage},
    state::AppState,
};

pub async fn on_message(
    message: ClientMessage,
    player_id: i32,
    state: &AppState,
) -> Option<ServerMessage> {
    dbg!(message);
    dbg!(player_id);
    dbg!(state);

    Some(ServerMessage::MatchFound)
}
