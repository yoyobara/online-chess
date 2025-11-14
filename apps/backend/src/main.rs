use crate::state::AppState;

mod configs;
mod constants;
mod extractors;
mod routes;
mod state;
mod utils;

const ADDR: (&str, u16) = ("0.0.0.0", 3000);

#[tokio::main]
async fn main() {
    let app = routes::router().with_state(AppState::default());
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
