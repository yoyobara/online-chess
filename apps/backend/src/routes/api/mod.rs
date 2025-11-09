use axum::{routing::get, Router};

mod example;

pub fn router() -> Router {
    Router::new().route("/echo", get(example::echo))
}
