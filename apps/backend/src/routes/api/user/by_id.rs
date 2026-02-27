use crate::{error::ApiResult, extractors::AuthUser, models::user::UserData, state::AppState};
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn by_id_handler(
    _: AuthUser,
    State(state): State<AppState>,
    Path(player_id): Path<i32>,
) -> ApiResult<Json<UserData>> {
    let user = state.user_repo.get_user(player_id).await?;

    Ok(Json(UserData::from(user)))
}
