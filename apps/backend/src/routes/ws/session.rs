use axum::extract::ws::WebSocket;
use sqlx::{Pool, Postgres};
use tokio::sync::broadcast::Receiver;

use crate::{
    internal_broadcast::InternalMessageWithReciever,
    routes::ws::{message::ServerMessage, receiver::SessionCommunicator},
};

enum SessionState {
    Connected,
}

pub struct Session {
    communicator: SessionCommunicator,
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
            communicator: SessionCommunicator::new(
                socket,
                internal_reciever,
                pool.clone(),
                player_id,
            ),
            player_id,
            pool,
            state: SessionState::Connected,
        }
    }

    pub async fn mainloop(&mut self) {
        loop {
            self.communicator.recv().await;

            self.communicator.ws_send(ServerMessage::MatchFound).await;
        }
    }
}
