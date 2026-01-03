mod configs;
mod constants;
mod error;
mod extractors;
mod matchmaking;
mod models;
mod repositories;
mod routes;
mod state;
mod utils;

use std::sync::Arc;

use tokio::{join, net::TcpListener};

use crate::{
    configs::{load_env, load_pool, load_redis},
    matchmaking::init_matchmaking_listener,
    repositories::user::SqlxUserRepository,
    state::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_env()?;
    let port = config.port;

    let pool = load_pool(&config.database_url).await?;
    let client = load_redis(&config.redis_url).await?;
    let redis_connection = client.get_multiplexed_async_connection().await?;

    let user_repo = Arc::new(SqlxUserRepository::new(pool.clone()));

    let (matchmaking_task, matchmaking_registry_map) = init_matchmaking_listener(client).await?;

    let state = AppState::new(
        config,
        user_repo,
        matchmaking_registry_map,
        redis_connection,
    )
    .await;

    let app = routes::router().with_state(state);
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    join!(matchmaking_task).0??;

    Ok(())
}
