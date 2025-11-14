use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::extractors::AuthUser;

pub async fn me_handler(AuthUser { player_id }: AuthUser) -> impl IntoResponse {
    Json(json!({
        "player_id": player_id
    }))
}
