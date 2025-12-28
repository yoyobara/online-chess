mod me;
mod rank;

use axum::{routing::get, Router};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(me::me_handler))
        .route("/rank", get(rank::rank_handler))
}
