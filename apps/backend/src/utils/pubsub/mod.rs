pub mod redis;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::mpsc::Receiver;

#[async_trait]
pub trait PubSub<T>: Send + Sync
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    async fn publish(&self, topic: &str, payload: &T) -> anyhow::Result<()>;
    async fn subscribe(&self, topic: &str) -> anyhow::Result<Receiver<T>>;
}

pub type PubSubFactory<T> = dyn Fn() -> Box<dyn PubSub<T>> + Send + Sync;
