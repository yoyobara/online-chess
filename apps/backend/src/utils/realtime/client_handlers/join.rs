use crate::{
    models::r#match::JoinResponse,
    utils::realtime::{client_communication::message::ServerMessage, RealtimeSession},
};

pub async fn handle_client_join(session: &mut RealtimeSession) -> anyhow::Result<()> {
    let match_state = session
        .app_state
        .match_repo
        .get_match_state(&session.match_id)
        .await?;

    session
        .communicator
        .send(ServerMessage::JoinResponse(JoinResponse {
            initial_state: match_state,
            color: session.player_color,
            opponent_id: session.opponent_id,
        }))
        .await
}
