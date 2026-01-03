use crate::{extractors::AuthUser, state::AppState, utils::new_uuid_v4};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use redis::AsyncTypedCommands;
use tokio::sync::oneshot;

#[allow(unused_variables)]
async fn handle_socket(
    mut socket: WebSocket,
    player_id: i32,
    mut app_state: AppState,
) -> anyhow::Result<()> {
    let player_pop_result: Option<i32> = app_state
        .redis_connection
        .lpop("matchmaking:waiting_players", None)
        .await?;

    if let Some(popped_player) = player_pop_result {
        let new_match_id = new_uuid_v4();

        app_state
            .redis_connection
            .set(format!("matches:{}", new_match_id), 1)
            .await?;

        app_state
            .redis_connection
            .publish(
                format!("matchmaking_waiting_users:{}", popped_player),
                &new_match_id,
            )
            .await?;

        socket.send(Message::Text(new_match_id.into())).await?;
    } else {
        let (tx, rx) = oneshot::channel::<String>();
        app_state.matchmaking_registry_map.insert(player_id, tx);
        app_state
            .redis_connection
            .lpush("matchmaking:waiting_players", player_id)
            .await?;

        let found_match_id = rx.await?;
        socket.send(Message::Text(found_match_id.into())).await?;
    }

    Ok(())
}

pub async fn realtime_handler(
    ws: WebSocketUpgrade,
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| handle_socket(socket, player_id, state).await.unwrap())
}
