use async_trait::async_trait;
use futures_util::StreamExt;
use redis::{AsyncTypedCommands, Client};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use tokio::sync::mpsc::{self, Receiver};

use crate::utils::pubsub::PubSub;

pub struct RedisPubSub<T> {
    client: Client,
    _phantom: PhantomData<T>,
}

impl<T> RedisPubSub<T> {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<T> PubSub<T> for RedisPubSub<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    async fn publish(&self, topic: &str, payload: &T) -> anyhow::Result<()> {
        let data = serde_json::to_vec(payload)?;
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        conn.publish(topic, data).await?;
        Ok(())
    }

    async fn subscribe(&self, topic: &str) -> anyhow::Result<Receiver<T>> {
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

                let payload_bytes: Vec<u8> = msg.get_payload()?;

                if let Ok(payload) = serde_json::from_slice(&payload_bytes) {
                    if tx.send(payload).await.is_err() {
                        break;
                    }
                }
            }

            Ok::<(), anyhow::Error>(())
        });

        Ok(rx)
    }
}
