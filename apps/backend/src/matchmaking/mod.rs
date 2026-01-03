mod common;
mod postman_task;

pub use common::RegistryMap;

use dashmap::DashMap;
use postman_task::matchmaking_postman_task;
use redis::Client;
use tokio::task::JoinHandle;

pub async fn init_matchmaking_listener(
    client: Client,
) -> anyhow::Result<(JoinHandle<anyhow::Result<()>>, RegistryMap)> {
    let registry_map = RegistryMap::new(DashMap::new());

    let matchmaking_postman = tokio::spawn(matchmaking_postman_task(client, registry_map.clone()));

    Ok((matchmaking_postman, registry_map))
}
