mod communicator;
mod handlers;
mod message;
mod session;
mod state;

use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
};

use crate::{extractors::AuthUser, routes::ws::session::Session, state::AppState};

async fn handle_socket(socket: WebSocket, player_id: i32, app_state: AppState) {
    let reciever = app_state.internal_sender.subscribe();
    let mut session = Session::new(socket, reciever, player_id, app_state.pool).await;

    session.mainloop().await;
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    println!("got ws request");
    ws.on_upgrade(async move |socket| handle_socket(socket, player_id, state).await)
}
