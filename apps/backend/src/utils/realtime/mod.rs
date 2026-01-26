pub mod client_communication;

mod client_handlers;
mod pubsub_handlers;

use tokio::select;

use crate::{
    state::AppState,
    utils::{
        pubsub::{message::PubSubMessage, PubSub},
        realtime::{
            client_communication::{message::ClientMessage, ClientCommunicator},
            client_handlers::handle_client_join,
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
        println!("{:?}", msg);

        Ok(())
    }

    async fn handle_client_msg(&mut self, msg: ClientMessage) -> anyhow::Result<()> {
        match msg {
            ClientMessage::JoinGame => handle_client_join(self).await,
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
