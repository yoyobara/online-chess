use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

use crate::{extractors::AuthUser, state::AppState};

pub async fn me_handler(
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user = sqlx::query!(
        "SELECT id, username, email, rank FROM users WHERE id = $1",
        player_id
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    Json(json!({
        "id": user.id,
        "username": user.username,
        "email": user.email,
        "rank": user.rank
    }))
}
