use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn load_pool(database_url: &str) -> anyhow::Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .connect(database_url)
        .await
        .map_err(Into::into)
}
