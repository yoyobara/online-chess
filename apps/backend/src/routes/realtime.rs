use crate::{
    error::{ApiError, ApiResult},
    extractors::AuthUser,
    state::AppState,
};
use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{ws::WebSocket, Path, State, WebSocketUpgrade},
    response::Response,
};
use redis::AsyncTypedCommands;

#[allow(unused_variables)]
async fn handle_socket(
    socket: WebSocket,
    player_id: i32,
    match_id: String,
    app_state: AppState,
) -> anyhow::Result<()> {
    unimplemented!()
}

pub async fn realtime_handler(
    ws: WebSocketUpgrade,
    AuthUser { player_id }: AuthUser,
    State(mut state): State<AppState>,
    Path(match_id): Path<String>,
) -> ApiResult<Response<Body>> {
    state
        .redis_connection
        .sismember(format!("player:{}:matches", player_id), &match_id)
        .await
        .map_err(|e| ApiError::from(anyhow!(e)))
        .and_then(|exists| {
            if exists {
                Ok(ws.on_upgrade(async move |socket| {
                    handle_socket(socket, player_id, match_id, state)
                        .await
                        .unwrap()
                }))
            } else {
                Err(ApiError::UserNotInMatch)
            }
        })
}
