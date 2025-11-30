mod client_state;
mod handlers;
mod message;
mod messager;

use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
};
use sqlx::postgres::PgListener;

use crate::{
    extractors::AuthUser,
    routes::ws::{
        client_state::ClientState, handlers::handle_looking_for_match, message::ClientMessage,
        messager::WsMessenger,
    },
    state::AppState,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| handle_socket(socket, player_id, state).await)
}

pub async fn handle_socket(socket: WebSocket, player_id: i32, app_state: AppState) {
    let mut messenger = WsMessenger::new(socket);
    let mut listener = PgListener::connect_with(&app_state.pool).await.unwrap();
    let mut client_state = ClientState::Connected;

    while let Some(msg) = messenger.recv().await {
        match msg {
            ClientMessage::LookingForMatch => {
                handle_looking_for_match(
                    &mut messenger,
                    &mut client_state,
                    &mut listener,
                    &app_state,
                    player_id,
                )
                .await
            }
        }
    }
}
