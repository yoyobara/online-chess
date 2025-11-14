use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{extractors::AuthUser, utils::create_auth_cookie};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login_handler(
    cookies: Cookies,
    Json(login_request): Json<LoginRequest>,
) -> impl IntoResponse {
    if login_request.username != login_request.password {
        return StatusCode::UNAUTHORIZED;
    }

    cookies.add(create_auth_cookie(AuthUser {
        player_id: "BRUH".to_owned(),
    }));

    StatusCode::OK
}
