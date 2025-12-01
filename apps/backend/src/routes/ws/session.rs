use axum::extract::ws::{self, WebSocket};
use sqlx::{Pool, Postgres};
use tokio::{select, sync::broadcast::Receiver};

use crate::routes::ws::message::{
    ClientMessage, Event, InternalMessage, InternalMessageWithReciever,
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
            let msg = self.internal_reciever.recv().await.ok()?;

            if msg.to_user == self.player_id {
                Some(msg.message)
            } else {
                None
            }
        };

        let ws_recv = async {
            let msg = self.socket.recv().await?.unwrap();
            let msg_text = msg.into_text().unwrap();

            Some(serde_json::from_str::<ClientMessage>(&msg_text).unwrap())
        };

        select! {
            Some(res) = internal_recv => Event::InternalMessage(res),
            Some(res) = ws_recv => Event::ClientMessage(res),
        }
    }
}
