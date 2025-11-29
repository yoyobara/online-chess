mod handler;
mod message;

use axum::{
    extract::{ws::Message::*, State, WebSocketUpgrade},
    response::IntoResponse,
};

use crate::{extractors::AuthUser, routes::ws::handler::on_message, state::AppState};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |mut socket| {
        while let Some(Ok(msg)) = socket.recv().await {
            match msg {
                Text(text) => {
                    on_message(serde_json::from_str(&text).unwrap(), player_id, &state).await
                }
                Close(_) => break,
                _ => {}
            };
        }
    })
}
