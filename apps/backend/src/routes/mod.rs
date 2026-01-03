mod api;
mod matchmaking;

use axum::{routing::any, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber;

use crate::{routes::matchmaking::matchmaking_handler, state::AppState};

pub fn router() -> Router<AppState> {
    tracing_subscriber::fmt::init();

    Router::new()
        .nest("/api", api::router())
        .route("/matchmaking", any(matchmaking_handler))
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
}
