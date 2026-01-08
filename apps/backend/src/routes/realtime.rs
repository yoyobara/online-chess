use crate::{
    error::{ApiError, ApiResult},
    extractors::AuthUser,
    state::AppState,
};
use axum::{
    body::Body,
    extract::{ws::WebSocket, Path, State, WebSocketUpgrade},
    response::Response,
};

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
    State(app_state): State<AppState>,
    Path(match_id): Path<String>,
) -> ApiResult<Response<Body>> {
    let in_match = app_state
        .match_repo
        .is_player_in_match(player_id, &match_id)
        .await?;

    in_match
        .then_some(ws.on_upgrade(async move |socket| {
            handle_socket(socket, player_id, match_id, app_state)
                .await
                .unwrap()
        }))
        .ok_or(ApiError::UserNotInMatch)
}
