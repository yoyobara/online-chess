use futures_util::StreamExt as _;
use redis::Client;

use crate::utils::matchmaking::RegistryMap;

fn extract_user_id_from_channel(channel_name: &str) -> i32 {
    let (_, user_id) = channel_name.split_once(":").unwrap();

    user_id.parse().unwrap()
}

pub async fn matchmaking_postman_task(
    client: Client,
    registry_map: RegistryMap,
) -> anyhow::Result<()> {
    let mut pubsub = client.get_async_pubsub().await?;
    pubsub.psubscribe("matchmaking_waiting_users:*").await?;

    let mut messages_stream = pubsub.into_on_message();
    while let Some(msg) = messages_stream.next().await {
        let user_id = extract_user_id_from_channel(msg.get_channel_name());

        if let Some((_, client)) = registry_map.remove(&user_id) {
            client
                .send(msg.get_payload()?)
                .map_err(|v| anyhow::anyhow!("can't send {}", v))?;
        }
    }

    Ok(())
}
