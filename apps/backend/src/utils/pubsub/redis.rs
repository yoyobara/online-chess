use async_trait::async_trait;
use futures_util::StreamExt;
use redis::{AsyncTypedCommands, Client};
use tokio::sync::mpsc::{self, Receiver};

use crate::utils::pubsub::PubSub;

pub struct RedisPubSub {
    client: Client,
}

impl RedisPubSub {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl PubSub for RedisPubSub {
    async fn publish(&self, topic: &str, payload: &[u8]) -> anyhow::Result<()> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        conn.publish(topic, payload).await?;
        Ok(())
    }

    async fn subscribe(&self, topic: &str) -> anyhow::Result<Receiver<Vec<u8>>> {
        let (tx, rx) = mpsc::channel(1024);

        let client_clone = self.client.clone();
        let topic = topic.to_string();

        tokio::spawn(async move {
            let mut pubsub = client_clone.get_async_pubsub().await?;
            pubsub.subscribe(topic.as_str()).await?;

            let mut on_message = pubsub.into_on_message();

            loop {
                let Some(msg) = on_message.next().await else {
                    break;
                };

                let payload: Vec<u8> = msg.get_payload()?;

                if tx.send(payload).await.is_err() {
                    break;
                }
            }

            anyhow::Ok(())
        });

        Ok(rx)
    }
}
