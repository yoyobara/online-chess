mod configs;
mod constants;
mod error;
mod extractors;
mod models;
mod repositories;
mod routes;
mod state;
mod utils;

use std::sync::Arc;

use tokio::net::TcpListener;

use crate::{
    configs::{load_env, load_pool, load_redis},
    repositories::{r#match::RedisMatchRepository, user::SqlxUserRepository},
    state::AppState,
    utils::pubsub::{redis::RedisPubSub, PubSubFactory},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_env()?;
    let port = config.port;

    let pool = load_pool(&config.database_url).await?;
    let client = load_redis(&config.redis_url).await?;

    let user_repo = Arc::new(SqlxUserRepository::new(pool.clone()));
    let match_repo = Arc::new(RedisMatchRepository::new(
        client.get_multiplexed_async_connection().await?,
    ));

    let pubsub_factory: Arc<PubSubFactory<String>> =
        Arc::new(move || Box::new(RedisPubSub::<String>::new(client.clone())));

    let state = AppState::new(config, pubsub_factory, user_repo, match_repo).await;

    let app = routes::router().with_state(state);
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
