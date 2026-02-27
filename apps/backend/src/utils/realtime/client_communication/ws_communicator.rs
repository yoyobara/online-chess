use async_trait::async_trait;
use axum::extract::ws::{Message, WebSocket};

use crate::utils::realtime::client_communication::message::{ClientMessage, ServerMessage};
use crate::utils::realtime::client_communication::ClientCommunicator;

pub struct WsCommunicator {
    socket: WebSocket,
}

impl WsCommunicator {
    pub fn new(client_ws: WebSocket) -> Self {
        WsCommunicator { socket: client_ws }
    }
}

#[async_trait]
impl ClientCommunicator for WsCommunicator {
    async fn send(&mut self, msg: ServerMessage) -> anyhow::Result<()> {
        let serialized = serde_json::to_string(&msg)?;

        self.socket
            .send(Message::text(serialized))
            .await
            .map_err(Into::into)
    }

    async fn recv(&mut self) -> Option<anyhow::Result<ClientMessage>> {
        let raw = self.socket.recv().await;

        raw.map(|res| {
            res.map_err(|e| anyhow::anyhow!(e)) // Convert socket errors to anyhow
                .and_then(|msg| {
                    let text = msg.to_text()?;
                    Ok(serde_json::from_str::<ClientMessage>(&text)?)
                })
        })
    }
}
