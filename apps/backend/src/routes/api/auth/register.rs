use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{
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
    Json(register_request): Json<RegisterRequest>,
) -> impl IntoResponse {
    let password_hash = hash_password(&register_request.password.into());

    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $2 OR username = $1;",
        register_request.username,
        register_request.email
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    if existing_user.is_some() {
        return StatusCode::CONFLICT;
    }

    let insert_query = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id;",
        register_request.username,
        register_request.email,
        password_hash
    )
    .fetch_one(&state.pool)
    .await
    .unwrap();

    cookies.add(create_auth_cookie(
        AuthUser {
            player_id: insert_query.id,
        },
        state.config.jwt_secret.as_ref(),
    ));

    StatusCode::CREATED
}
