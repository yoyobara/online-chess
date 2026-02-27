use async_trait::async_trait;

use crate::utils::realtime::client_communication::message::{ClientMessage, ServerMessage};

#[async_trait]
pub trait ClientCommunicator: Send {
    async fn send(&mut self, msg: ServerMessage) -> anyhow::Result<()>;
    async fn recv(&mut self) -> Option<anyhow::Result<ClientMessage>>;
}
