use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{
    error::ApiResult,
    extractors::AuthUser,
    state::AppState,
    utils::{create_auth_cookie, hash_password},
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub async fn register_handler(
    cookies: Cookies,
    State(state): State<AppState>,
    Json(RegisterRequest {
        username,
        email,
        password,
    }): Json<RegisterRequest>,
) -> ApiResult<StatusCode> {
    let password_hash = hash_password(&password.into());

    let created_user_id = state
        .user_repo
        .create_user(username, email, password_hash)
        .await?;

    cookies.add(create_auth_cookie(
        AuthUser {
            player_id: created_user_id,
        },
        state.config.jwt_secret.as_ref(),
    ));

    Ok(StatusCode::CREATED)
}
