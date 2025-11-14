use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{extractors::AuthUser, state::AppState, utils::create_auth_cookie};

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
    if login_request.username != login_request.password {
        return StatusCode::UNAUTHORIZED;
    }

    cookies.add(create_auth_cookie(
        AuthUser {
            player_id: "BRUH".to_owned(),
        },
        state.config.jwt_secret.as_ref(),
    ));

    StatusCode::OK
}
