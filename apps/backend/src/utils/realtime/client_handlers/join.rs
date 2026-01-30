use anyhow::anyhow;
use rust_chess::core::color::Color;

use crate::{
    models::r#match::JoinResponse,
    utils::realtime::{client_communication::message::ServerMessage, RealtimeSession},
};

pub async fn handle_client_join(session: &mut RealtimeSession) -> anyhow::Result<()> {
    session
        .app_state
        .match_repo
        .set_player_connected(&session.match_id, session.player_id, true)
        .await?;

    let match_state = session
        .app_state
        .match_repo
        .get_match_state(&session.match_id)
        .await?;

    let players = session
        .app_state
        .match_repo
        .get_players(&session.match_id)
        .await?;

    let (player_color, opponent_id) = match session.player_id {
        id if id == players.white_player_id => (Color::White, players.black_player_id),
        id if id == players.black_player_id => (Color::Black, players.white_player_id),
        _ => return Err(anyhow!("player not in match!")),
    };

    session
        .communicator
        .send(ServerMessage::JoinResponse(JoinResponse {
            initial_state: match_state,
            color: player_color,
            opponent_id,
        }))
        .await
}
