use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub async fn register_handler(
    cookies: Cookies,
    Json(register_request): Json<RegisterRequest>,
) -> impl IntoResponse {
    StatusCode::OK
}
