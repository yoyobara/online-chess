mod login;
mod me;
mod register;

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/me", get(me::me_handler))
        .route("/login", post(login::login_handler))
        .route("/register", post(register::register_handler))
}
