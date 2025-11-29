mod api;
mod ws;

use axum::{routing::any, Router};
use tower_cookies::CookieManagerLayer;

use crate::{routes::ws::ws_handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/api", api::router())
        .route("/ws", any(ws_handler))
        .layer(CookieManagerLayer::new())
}
