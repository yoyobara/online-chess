mod by_id;
mod me;

use axum::{routing::get, Router};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(me::me_handler))
        .route("/{id}", get(by_id::by_id_handler))
}
