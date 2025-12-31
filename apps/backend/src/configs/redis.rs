use redis::aio::{MultiplexedConnection, PubSub};

pub async fn load_redis(client_url: &str) -> anyhow::Result<(MultiplexedConnection, PubSub)> {
    let client = redis::Client::open(client_url)?;

    let data_conn = client.get_multiplexed_async_connection().await?;
    let pubsub_conn = client.get_async_pubsub().await?;

    Ok((data_conn, pubsub_conn))
}
