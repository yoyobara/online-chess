use axum::{http::StatusCode, response::IntoResponse};
use tower_cookies::{Cookie, Cookies};

use crate::{constants::auth::AUTH_COOKIE_NAME, extractors::AuthUser};

pub async fn logout_handler(_: AuthUser, cookies: Cookies) -> impl IntoResponse {
    let cookie = Cookie::build(AUTH_COOKIE_NAME)
        .http_only(true)
        .path("/")
        .build();

    cookies.remove(cookie);

    StatusCode::OK
}
