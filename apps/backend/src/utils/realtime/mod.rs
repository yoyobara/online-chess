pub mod client_communication;

mod client_handlers;
mod pubsub_handlers;

use anyhow::anyhow;
use tokio::select;

use crate::{
    models::r#match::MatchState,
    state::AppState,
    utils::{
        pubsub::{message::PubSubMessage, PubSub},
        realtime::{
            client_communication::{
                message::{ClientMessage, ServerMessage},
                ClientCommunicator,
            },
            client_handlers::{handle_client_join, handle_client_player_move},
        },
    },
};

pub struct RealtimeSession {
    app_state: AppState,
    player_id: i32,
    match_id: String,
    communicator: Box<dyn ClientCommunicator>,
    pubsub: Box<dyn PubSub>,
}

impl RealtimeSession {
    pub fn new(
        app_state: AppState,
        communicator: Box<dyn ClientCommunicator>,
        player_id: i32,
        match_id: String,
    ) -> Self {
        let pubsub = (app_state.pubsub_factory)();

        Self {
            app_state,
            player_id,
            match_id,
            communicator,
            pubsub,
        }
    }

    async fn handle_pubsub_msg(&mut self, msg: PubSubMessage) -> anyhow::Result<()> {
        match msg {
            PubSubMessage::PlayerMove(new_state) => {
                self.communicator
                    .send(ServerMessage::NewState(new_state))
                    .await
            }
            _ => Err(anyhow!("bad pubsub message")),
        }
    }

    async fn handle_client_msg(&mut self, msg: ClientMessage) -> anyhow::Result<()> {
        match msg {
            ClientMessage::JoinGame => handle_client_join(self).await,
            ClientMessage::PlayerMove(move_data) => {
                handle_client_player_move(self, move_data).await
            }
        }
    }

    pub async fn mainloop(mut self) -> anyhow::Result<()> {
        let mut pubsub_reciever = self
            .pubsub
            .subscribe(&format!("match:{}", self.match_id))
            .await?;

        loop {
            select! {
                Some(pubsub_msg) = pubsub_reciever.recv() => self.handle_pubsub_msg(pubsub_msg?).await?,
                Some(client_msg) = self.communicator.recv() => self.handle_client_msg(client_msg?).await?,
                else => {
                    break;
                }
            }
        }

        Ok(())
    }
}
