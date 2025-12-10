use axum::extract::ws::{Message, WebSocket};
use sqlx::{Pool, Postgres};
use tokio::select;
use tokio::sync::broadcast::Receiver;

use crate::{
    internal_broadcast::{InternalMessage, InternalMessageWithMetadata},
    routes::ws::message::{ClientMessage, ServerMessage},
};

#[derive(Debug)]
pub enum Event {
    ClientMessage(ClientMessage),
    InternalMessage(InternalMessage),
}

pub struct SessionCommunicator {
    socket: WebSocket,
    internal_reciever: Receiver<InternalMessageWithMetadata>,
    pool: Pool<Postgres>,
    player_id: i32,
}

impl SessionCommunicator {
    pub fn new(
        socket: WebSocket,
        internal_reciever: Receiver<InternalMessageWithMetadata>,
        pool: Pool<Postgres>,
        player_id: i32,
    ) -> Self {
        Self {
            socket,
            internal_reciever,
            pool,
            player_id,
        }
    }

    pub async fn recv(&mut self) -> Option<Event> {
        let internal_recv = async {
            loop {
                let msg = self.internal_reciever.recv().await.unwrap();

                if msg.to_user == self.player_id {
                    println!(
                        "INTERNAL_RECV {} FROM {}: {:?}",
                        self.player_id, msg.from_user, msg.message
                    );

                    return msg.message;
                }
            }
        };

        let ws_recv = async {
            let msg = self.socket.recv().await?.unwrap();

            let parsed_message = match msg {
                Message::Close(_) => ClientMessage::ConnectionClosed,
                Message::Text(text) => serde_json::from_str::<ClientMessage>(&text).unwrap(),
                _ => unimplemented!(),
            };

            println!("WS client -> {}: {:?}", self.player_id, parsed_message);

            Some(parsed_message)
        };

        select! {
            internal_msg = internal_recv => Some(Event::InternalMessage(internal_msg)),
            ws_msg = ws_recv => ws_msg.map(Event::ClientMessage),
        }
    }

    pub async fn internal_notify(&mut self, user_target: i32, message: InternalMessage) {
        let msg = InternalMessageWithMetadata {
            from_user: self.player_id,
            to_user: user_target,
            message,
        };

        let serialized = serde_json::to_string::<InternalMessageWithMetadata>(&msg).unwrap();

        sqlx::query!("SELECT pg_notify('game_events', $1);", serialized)
            .execute(&self.pool)
            .await
            .unwrap();

        println!(
            "INTERNAL_SEND {} TO {}: {:?}",
            self.player_id, msg.to_user, msg.message
        );
    }

    pub async fn ws_send(&mut self, message: ServerMessage) {
        let serialized = serde_json::to_string::<ServerMessage>(&message).unwrap();

        self.socket.send(Message::text(serialized)).await.unwrap();

        println!("WS {} -> client: {:?}", self.player_id, message);
    }

    pub async fn ws_log(&mut self, log_message: impl Into<String>) {
        self.ws_send(ServerMessage::Log {
            message: log_message.into(),
        })
        .await;
    }
}
