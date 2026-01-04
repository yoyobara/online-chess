use crate::{extractors::AuthUser, state::AppState, utils::uuid::new_uuid_v4};
use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
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
    let match_id;
    let player_pop_result: Option<i32> = app_state
        .redis_connection
        .lpop("matchmaking:waiting_players", None)
        .await?;

    if let Some(popped_player) = player_pop_result {
        match_id = new_uuid_v4();

        let _: () = redis::pipe()
            .atomic()
            .hset_multiple(
                format!("matches:{}", &match_id),
                &[("player1_id", player_id), ("player2_id", popped_player)],
            )
            .sadd(format!("player:{}:matches", player_id), &match_id)
            .sadd(format!("player:{}:matches", popped_player), &match_id)
            .query_async(&mut app_state.redis_connection)
            .await?;

        app_state
            .redis_connection
            .publish(
                format!("matchmaking_waiting_users:{}", popped_player),
                &match_id,
            )
            .await?;
    } else {
        let (tx, rx) = oneshot::channel::<String>();
        app_state.matchmaking_registry_map.insert(player_id, tx);
        app_state
            .redis_connection
            .lpush("matchmaking:waiting_players", player_id)
            .await?;

        match_id = rx.await?;
    }

    socket
        .send(serde_json::to_string(&match_id)?.into())
        .await?;

    Ok(())
}

pub async fn matchmaking_handler(
    ws: WebSocketUpgrade,
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |socket| handle_socket(socket, player_id, state).await.unwrap())
}
