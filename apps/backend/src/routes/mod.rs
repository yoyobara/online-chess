mod api;

use axum::Router;
use tower_cookies::CookieManagerLayer;

pub fn router() -> Router {
    Router::new()
        .nest("/api", api::router())
        .layer(CookieManagerLayer::new())
}
