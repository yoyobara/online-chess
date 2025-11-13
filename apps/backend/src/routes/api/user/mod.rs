mod me;

use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/me", get(me::me_handler))
}
