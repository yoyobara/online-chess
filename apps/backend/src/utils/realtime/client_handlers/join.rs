use crate::{
    models::r#match::JoinResponse,
    utils::realtime::{client_communication::message::ServerMessage, RealtimeSession},
};

pub async fn handle_client_join(session: &mut RealtimeSession) -> anyhow::Result<()> {
    let match_state = session
        .app_state
        .ephemeral_match_repo
        .get_match_state(&session.match_id)
        .await?;

    let initial_moves = session
        .app_state
        .ephemeral_match_repo
        .get_match_moves(&session.match_id)
        .await?;

    session
        .communicator
        .send(ServerMessage::JoinResponse(JoinResponse {
            initial_state: match_state,
            color: session.player_color,
            opponent_id: session.opponent_id,
            initial_moves: initial_moves.into_iter().map(Into::into).collect(),
        }))
        .await
}
