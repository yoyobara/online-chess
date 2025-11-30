use axum::extract::ws::{Message, WebSocket};

use crate::routes::ws::message::{ClientMessage, ServerMessage};

pub struct WsMessenger {
    pub socket: WebSocket,
}

impl WsMessenger {
    pub fn new(socket: WebSocket) -> Self {
        Self { socket }
    }

    pub async fn recv(&mut self) -> Option<ClientMessage> {
        let msg = self.socket.recv().await?.unwrap();

        serde_json::from_str(&msg.into_text().ok()?).unwrap()
    }

    pub async fn send(&mut self, msg: ServerMessage) {
        let serialized = serde_json::to_string(&msg).unwrap();

        self.socket.send(Message::text(serialized)).await.unwrap();
    }
}
