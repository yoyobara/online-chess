use axum::extract::ws::WebSocket;
use sqlx::{Pool, Postgres};
use tokio::sync::broadcast::Receiver;

use crate::{
    internal_broadcast::{InternalMessage, InternalMessageWithReciever},
    routes::ws::{
        communicator::{Event, SessionCommunicator},
        handlers::{handle_looking_for_match, handle_match_found},
        message::{ClientMessage, ServerMessage},
        state::SessionState,
    },
};

pub struct Session {
    pub(super) communicator: SessionCommunicator,
    pub(super) player_id: i32,
    pub(super) pool: Pool<Postgres>,
    pub(super) state: SessionState,
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

    async fn handle_client_message(&mut self, msg: ClientMessage) {
        match msg {
            ClientMessage::LookingForMatch => handle_looking_for_match(self).await,
        }
    }

    async fn handle_internal_message(&mut self, msg: InternalMessage) {
        match msg {
            InternalMessage::MatchFound {
                match_id,
                opponent_id,
            } => handle_match_found(self, match_id, opponent_id).await,
        }
    }

    pub(super) async fn client_log(&mut self, message: &'static str) {
        self.communicator
            .ws_send(ServerMessage::Log { message })
            .await;
    }
}
