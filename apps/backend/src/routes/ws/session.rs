use axum::extract::ws::WebSocket;
use sqlx::{Pool, Postgres};
use tokio::sync::broadcast::Receiver;

use crate::{
    internal_broadcast::{InternalMessage, InternalMessageWithReciever},
    routes::ws::{
        communicator::{Event, SessionCommunicator},
        message::ClientMessage,
        state::SessionState,
    },
};

pub struct Session {
    pub(super) communicator: SessionCommunicator,
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
            let event = self.communicator.recv().await;

            match event {
                Event::ClientMessage(client_msg) => self.handle_client_message(client_msg).await,
                Event::InternalMessage(internal_msg) => {
                    self.handle_internal_message(internal_msg).await
                }
            };
        }
    }

    async fn handle_client_message(&mut self, msg: ClientMessage) {}
    async fn handle_internal_message(&mut self, msg: InternalMessage) {}

    async fn handle_looking_for_match(&mut self) {
        assert_eq!(self.state, SessionState::Connected);
    }
}
