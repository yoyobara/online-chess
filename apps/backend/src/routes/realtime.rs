use crate::{
    error::{ApiError, ApiResult},
    extractors::AuthUser,
    state::AppState,
    utils::realtime::{client_communication::WsCommunicator, RealtimeSession},
};
use axum::{
    body::Body,
    extract::{ws::WebSocket, Path, State, WebSocketUpgrade},
    response::Response,
};
use tracing::error;

#[allow(unused_variables)]
async fn handle_socket(
    socket: WebSocket,
    player_id: i32,
    match_id: String,
    app_state: AppState,
) -> anyhow::Result<()> {
    let client_communicator = Box::new(WsCommunicator::new(socket));
    let session = RealtimeSession::new(app_state, client_communicator, player_id, match_id).await?;

    session.mainloop().await
}

pub async fn realtime_handler(
    ws: WebSocketUpgrade,
    AuthUser { player_id }: AuthUser,
    State(app_state): State<AppState>,
    Path(match_id): Path<String>,
) -> ApiResult<Response<Body>> {
    let in_match = app_state
        .ephemeral_match_repo
        .is_player_in_match(player_id, &match_id)
        .await?;

    if !in_match {
        return Err(ApiError::UserNotInMatch);
    }

    Ok(ws.on_upgrade(async move |socket| {
        if let Err(e) = handle_socket(socket, player_id, match_id, app_state).await {
            error!(error = %e, "Error in websocket session");
        }
    }))
}
