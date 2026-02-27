use crate::{error::ApiResult, extractors::AuthUser, models::user::UserData, state::AppState};
use axum::{extract::State, Json};

pub async fn me_handler(
    AuthUser { player_id }: AuthUser,
    State(state): State<AppState>,
) -> ApiResult<Json<UserData>> {
    let user = state.user_repo.get_user(player_id).await?;

    Ok(Json(UserData::from(user)))
}
