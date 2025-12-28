mod login;
mod logout;
mod register;

use axum::{routing::post, Router};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login::login_handler))
        .route("/register", post(register::register_handler))
        .route("/logout", post(logout::logout_handler))
}
