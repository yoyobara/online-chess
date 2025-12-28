mod api;
mod realtime;

use axum::{routing::any, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber;

use crate::{routes::realtime::realtime_handler, state::AppState};

pub fn router() -> Router<AppState> {
    tracing_subscriber::fmt::init();

    Router::new()
        .nest("/api", api::router())
        .route("/realtime", any(realtime_handler))
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
}
