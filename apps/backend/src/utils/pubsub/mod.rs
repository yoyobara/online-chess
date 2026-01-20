pub mod message;
pub mod redis;

use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::utils::pubsub::message::PubSubMessage;

#[async_trait]
pub trait PubSub: Send {
    async fn publish(&self, topic: &str, payload: &PubSubMessage) -> anyhow::Result<()>;
    async fn subscribe(
        &self,
        topic: &str,
    ) -> anyhow::Result<Receiver<anyhow::Result<PubSubMessage>>>;
}
