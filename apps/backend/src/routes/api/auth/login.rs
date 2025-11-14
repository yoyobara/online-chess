use std::time::{SystemTime, UNIX_EPOCH};

use axum::{http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use crate::{
    config::CONFIG,
    constants::auth::AUTH_COOKIE_NAME,
    extractors::{AuthUser, Claims},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[axum::debug_handler]
pub async fn login_handler(
    cookies: Cookies,
    Json(login_request): Json<LoginRequest>,
) -> impl IntoResponse {
    if login_request.username != login_request.password {
        return StatusCode::UNAUTHORIZED;
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        iat: timestamp,
        exp: timestamp + 60 * 60 * 24 * 7,
        user: AuthUser {
            player_id: "hel".to_owned(),
        },
    };

    let encoded = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
    )
    .unwrap();

    cookies.add(Cookie::new(AUTH_COOKIE_NAME, encoded));

    StatusCode::OK
}
