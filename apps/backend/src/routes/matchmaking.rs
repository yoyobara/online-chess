use crate::{extractors::AuthUser, state::AppState, utils::pubsub::message::PubSubMessage};
use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
};

#[allow(unused_variables)]
async fn handle_socket(
    mut socket: WebSocket,
    player_id: i32,
    app_state: AppState,
) -> anyhow::Result<()> {
    let pubsub = (app_state.pubsub_factory)();
    let match_id;
    let player_pop_result = app_state.match_repo.pop_matchmaking_player().await?;

    if let Some(popped_player) = player_pop_result {
        match_id = app_state
            .match_repo
            .register_match(popped_player, player_id)
            .await?;

        pubsub
            .publish(
                &format!("matchmaking_waiting_users:{}", popped_player),
                &PubSubMessage::MatchmakingMatchId(match_id.clone()),
            )
            .await?;
    } else {
        let mut rx = pubsub
            .subscribe(&format!("matchmaking_waiting_users:{}", player_id))
            .await?;

        app_state
            .match_repo
            .push_matchmaking_player(player_id)
            .await?;

        match_id = match rx.recv().await.unwrap() {
            PubSubMessage::MatchmakingMatchId(id) => id,
        };
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
