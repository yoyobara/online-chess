mod configs;
mod constants;
mod error;
mod extractors;
mod internal_broadcast;
mod models;
mod repositories;
mod routes;
mod state;
mod utils;

use std::sync::Arc;

use tokio::{net::TcpListener, select, signal};

use crate::{
    configs::{load_env, load_pool},
    internal_broadcast::start_internal_broadcast,
    repositories::user::SqlxUserRepository,
    state::AppState,
};

const ADDR: (&str, u16) = ("0.0.0.0", 3000);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_env()?;
    let pool = load_pool(&config.database_url).await?;
    let user_repo = Arc::new(SqlxUserRepository::new(pool.clone()));
    let (tx, internal_broadcast_task) = start_internal_broadcast(pool.clone()).await;

    let state = AppState::new(config, tx, user_repo).await;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = routes::router().with_state(state);
    let listener = TcpListener::bind(ADDR).await?;
    let server = axum::serve(listener, app);

    select! {
        _ = server => {
            // should never run, server never ends.
            eprintln!("server quitted unexpectedly...")
        },
        res = internal_broadcast_task => res??
    }

    Ok(())
}
