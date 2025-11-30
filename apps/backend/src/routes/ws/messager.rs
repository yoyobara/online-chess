use axum::extract::ws::{Message, WebSocket};

use crate::routes::ws::message::{ClientMessage, ServerMessage};

pub struct WsMessager {
    pub socket: WebSocket,
}

impl WsMessager {
    pub fn new(socket: WebSocket) -> Self {
        Self { socket }
    }

    pub async fn recv(&mut self) -> ClientMessage {
        let msg = self.socket.recv().await.unwrap().unwrap();

        serde_json::from_str(&msg.into_text().unwrap()).unwrap()
    }

    pub async fn send(&mut self, msg: ServerMessage) {
        let serialized = serde_json::to_string(&msg).unwrap();

        self.socket.send(Message::text(serialized)).await.unwrap();
    }
}
