use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{
    error::{ApiError, ApiResult},
    extractors::AuthUser,
    state::AppState,
    utils::auth::{create_auth_cookie, verify_password},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login_handler(
    cookies: Cookies,
    State(state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> ApiResult<StatusCode> {
    let user = state.user_repo.get_by_email(login_request.email).await?;

    if verify_password(&login_request.password.into(), &user.password_hash)? {
        cookies.add(create_auth_cookie(
            AuthUser { player_id: user.id },
            state.config.jwt_secret.as_ref(),
        )?);

        Ok(StatusCode::OK)
    } else {
        Err(ApiError::WrongPassword)
    }
}
