use crate::utils::realtime::{client_communication::message::ServerMessage, RealtimeSession};

pub async fn handle_client_join(session: &mut RealtimeSession) -> anyhow::Result<()> {
    let match_state = session
        .app_state
        .match_repo
        .get_match_state(&session.match_id)
        .await?;

    session
        .app_state
        .match_repo
        .set_player_connected(&session.match_id, session.player_id, true)
        .await?;

    session
        .communicator
        .send(ServerMessage::MatchState(match_state))
        .await
}
