use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
};

use crate::{extractors::AuthUser, state::AppState, utils::Session};

async fn handle_socket(socket: WebSocket, player_id: i32, app_state: AppState) {
    let reciever = app_state.internal_sender.subscribe();
    let mut session = Session::new(socket, reciever, player_id, app_state).await;

    session.mainloop().await;
}

pub async fn realtime_handler(
    ws: WebSocketUpgrade,
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| handle_socket(socket, player_id, state).await)
}
