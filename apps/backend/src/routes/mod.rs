use axum::Router;

mod api;

pub fn router() -> Router {
    Router::new().nest("/api", api::router())
}
