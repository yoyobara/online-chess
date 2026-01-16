pub mod redis;

use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

#[async_trait]
pub trait PubSub: Send + Sync {
    async fn publish(&self, topic: &str, payload: &[u8]) -> anyhow::Result<()>;
    async fn subscribe(&self, topic: &str) -> anyhow::Result<Receiver<Vec<u8>>>;
}

pub type PubSubFactory = dyn Fn() -> Box<dyn PubSub> + Send + Sync;
