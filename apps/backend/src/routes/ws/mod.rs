mod handler;
mod message;
mod messager;

use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};

use crate::{
    extractors::AuthUser,
    routes::ws::{handler::handle_socket, messager::WsMessager},
    state::AppState,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| {
        handle_socket(WsMessager::new(socket), player_id, state).await
    })
}
