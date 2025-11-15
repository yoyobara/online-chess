use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{
    extractors::AuthUser,
    state::AppState,
    utils::{create_auth_cookie, verify_password},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login_handler(
    cookies: Cookies,
    State(state): State<AppState>,
    Json(login_request): Json<LoginRequest>,
) -> impl IntoResponse {
    let user_query = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE username = ?",
        login_request.username
    )
    .fetch_one(&state.pool)
    .await;

    if let Ok(user) = user_query {
        if verify_password(&login_request.password.into(), &user.password_hash) {
            cookies.add(create_auth_cookie(
                AuthUser { player_id: user.id },
                state.config.jwt_secret.as_ref(),
            ));

            StatusCode::OK
        } else {
            StatusCode::UNAUTHORIZED
        }
    } else {
        StatusCode::NOT_FOUND
    }
}
