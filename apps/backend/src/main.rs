mod configs;
mod constants;
mod extractors;
mod internal_broadcast;
mod models;
mod repositories;
mod routes;
mod state;
mod utils;

use crate::{
    configs::{load_env, load_pool},
    internal_broadcast::start_internal_broadcast,
    state::AppState,
};

const ADDR: (&str, u16) = ("0.0.0.0", 3000);

#[tokio::main]
async fn main() {
    let config = load_env();
    let pool = load_pool(&config.database_url).await;
    let (tx, internal_broadcast_task) = start_internal_broadcast(pool.clone()).await;
    let state = AppState::new(config, pool, tx).await;

    let app = routes::router().with_state(state);
    let listener = tokio::net::TcpListener::bind(ADDR).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    internal_broadcast_task.await.unwrap();
}
