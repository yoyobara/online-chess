mod client_state;
mod handlers;
mod message;

use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
};

use crate::{extractors::AuthUser, state::AppState};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| handle_socket(socket, player_id, state).await)
}

pub async fn handle_socket(socket: WebSocket, player_id: i32, app_state: AppState) {}
