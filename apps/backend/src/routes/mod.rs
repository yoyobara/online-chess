mod api;

use axum::Router;
use tower_cookies::CookieManagerLayer;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/api", api::router())
        .layer(CookieManagerLayer::new())
}
