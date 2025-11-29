mod handler;
mod message;
mod messager;

use axum::{
    extract::{
        ws::Message::{self, *},
        State, WebSocketUpgrade,
    },
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
                    let to_send =
                        on_message(serde_json::from_str(&text).unwrap(), player_id, &state).await;

                    if let Some(res) = to_send {
                        socket
                            .send(Message::text(serde_json::to_string(&res).unwrap()))
                            .await
                            .unwrap();
                    }
                }
                Close(_) => break,
                _ => {}
            };
        }
    })
}
