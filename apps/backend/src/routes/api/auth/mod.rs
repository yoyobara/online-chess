mod login;
mod me;
mod register;

use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(me::me_handler))
        .route("/login", post(login::login_handler))
        .route("/register", post(register::register_handler))
}
