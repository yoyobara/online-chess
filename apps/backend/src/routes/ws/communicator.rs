use axum::extract::ws::{Message, WebSocket};
use sqlx::{Pool, Postgres};
use tokio::select;
use tokio::sync::broadcast::Receiver;

use crate::{
    internal_broadcast::{InternalMessage, InternalMessageWithReciever},
    routes::ws::message::{ClientMessage, ServerMessage},
};

#[derive(Debug)]
pub enum Event {
    ClientMessage(ClientMessage),
    InternalMessage(InternalMessage),
}

pub struct SessionCommunicator {
    socket: WebSocket,
    internal_reciever: Receiver<InternalMessageWithReciever>,
    pool: Pool<Postgres>,
    player_id: i32,
}

impl SessionCommunicator {
    pub fn new(
        socket: WebSocket,
        internal_reciever: Receiver<InternalMessageWithReciever>,
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

    pub async fn recv(&mut self) -> Event {
        let internal_recv = async {
            loop {
                let msg = self.internal_reciever.recv().await.unwrap();

                if msg.to_user == self.player_id {
                    return msg.message;
                }
            }
        };

        let ws_recv = async {
            let msg = self.socket.recv().await.unwrap();
            dbg!(&msg);
            let msg = msg.unwrap();

            let deserialized =
                serde_json::from_str::<ClientMessage>(&msg.into_text().unwrap()).unwrap();

            deserialized
        };

        select! {
            internal_msg = internal_recv => Event::InternalMessage(internal_msg),
            ws_msg = ws_recv => Event::ClientMessage(ws_msg),
        }
    }

    pub async fn internal_notify(&mut self, user_target: i32, message: InternalMessage) {
        let msg = InternalMessageWithReciever {
            to_user: user_target,
            message,
        };

        let serialized = serde_json::to_string::<InternalMessageWithReciever>(&msg).unwrap();

        sqlx::query!("SELECT pg_notify('game_events', $1);", serialized)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    pub async fn ws_send(&mut self, message: ServerMessage) {
        let serialized = serde_json::to_string::<ServerMessage>(&message).unwrap();

        self.socket.send(Message::text(serialized)).await.unwrap();
    }

    pub async fn ws_log(&mut self, log_message: impl Into<String>) {
        self.ws_send(ServerMessage::Log {
            message: log_message.into(),
        })
        .await;
    }
}
