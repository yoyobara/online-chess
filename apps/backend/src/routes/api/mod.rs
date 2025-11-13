mod user;

use axum::Router;

pub fn router() -> Router {
    Router::new().nest("/auth", user::router())
}
