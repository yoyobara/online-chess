use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub async fn load_pool(database_url: &str) -> Pool<Sqlite> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .connect(database_url)
        .await
        .unwrap()
}
