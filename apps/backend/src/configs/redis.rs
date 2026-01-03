use redis::Client;

pub async fn load_redis(client_url: &str) -> anyhow::Result<Client> {
    let client = redis::Client::open(client_url)?;

    Ok(client)
}
