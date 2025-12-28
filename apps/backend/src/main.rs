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
    configs::{load_env, load_pool},
    repositories::user::SqlxUserRepository,
    state::AppState,
};

const ADDR: (&str, u16) = ("0.0.0.0", 3000);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_env()?;
    let pool = load_pool(&config.database_url).await?;
    let user_repo = Arc::new(SqlxUserRepository::new(pool.clone()));

    let state = AppState::new(config, user_repo).await;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = routes::router().with_state(state);
    let listener = TcpListener::bind(ADDR).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
