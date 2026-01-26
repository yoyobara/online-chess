use crate::utils::realtime::RealtimeSession;

pub async fn handle_client_join(session: &mut RealtimeSession) -> anyhow::Result<()> {
    session
        .app_state
        .match_repo
        .set_player_connected(&session.match_id, session.player_id, true)
        .await
        .map_err(Into::into)
}
