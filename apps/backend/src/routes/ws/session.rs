use axum::extract::ws::{self, WebSocket};
use sqlx::{Pool, Postgres};
use tokio::{select, sync::broadcast::Receiver};

use crate::{
    internal_broadcast::InternalMessageWithReciever,
    routes::ws::message::{ClientMessage, Event},
};

enum SessionState {
    Connected,
}

pub struct Session {
    socket: WebSocket,
    internal_reciever: Receiver<InternalMessageWithReciever>,
    player_id: i32,
    pool: Pool<Postgres>,
    state: SessionState,
}

impl Session {
    pub async fn new(
        socket: WebSocket,
        internal_reciever: Receiver<InternalMessageWithReciever>,
        player_id: i32,
        pool: Pool<Postgres>,
    ) -> Self {
        Self {
            socket,
            internal_reciever,
            player_id,
            pool,
            state: SessionState::Connected,
        }
    }

    pub async fn mainloop(&mut self) {
        loop {
            let msg = self.recv().await;

            self.socket
                .send(ws::Message::text(format!("{:?}", msg)))
                .await
                .unwrap();
        }
    }

    async fn recv(&mut self) -> Event {
        let internal_recv = async {
            loop {
                let msg = self.internal_reciever.recv().await.unwrap();

                if msg.to_user == self.player_id {
                    return msg.message;
                }
            }
        };

        let ws_recv = async {
            let msg = self.socket.recv().await.unwrap().unwrap();

            let deserialized =
                serde_json::from_str::<ClientMessage>(&msg.into_text().unwrap()).unwrap();

            deserialized
        };

        select! {
            internal_msg = internal_recv => Event::InternalMessage(internal_msg),
            ws_msg = ws_recv => Event::ClientMessage(ws_msg),
        }
    }
}
