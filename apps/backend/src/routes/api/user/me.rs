use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::{error::ApiResult, extractors::AuthUser, state::AppState};

pub async fn me_handler(
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> ApiResult<Json<Value>> {
    let user = state.user_repo.get_user(player_id).await?;

    Ok(Json(json!({
        "id": user.id,
        "username": user.username,
    })))
}
