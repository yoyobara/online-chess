use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

use crate::{extractors::AuthUser, state::AppState};

pub async fn rank_handler(
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let rank = sqlx::query_scalar!("SELECT rank FROM users WHERE id = $1", player_id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    Json(json!(rank))
}
