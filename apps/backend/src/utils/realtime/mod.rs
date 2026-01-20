mod message;

use axum::extract::ws::{Message, WebSocket};
use tokio::select;

use crate::{
    state::AppState,
    utils::pubsub::{message::PubSubMessage, PubSub},
};

pub struct RealtimeSession {
    app_state: AppState,
    socket: WebSocket,
    player_id: i32,
    match_id: String,
    pubsub: Box<dyn PubSub>,
}

impl RealtimeSession {
    pub fn new(app_state: AppState, socket: WebSocket, player_id: i32, match_id: String) -> Self {
        let pubsub = (app_state.pubsub_factory)();

        Self {
            app_state,
            socket,
            player_id,
            match_id,
            pubsub,
        }
    }

    async fn handle_pubsub_msg(&mut self, msg: PubSubMessage) {}

    async fn handle_client_msg(&mut self, msg: Message) {}

    pub async fn mainloop(mut self) -> anyhow::Result<()> {
        let mut pubsub_reciever = self
            .pubsub
            .subscribe(&format!("match:{}", self.match_id))
            .await?;

        loop {
            select! {
                Some(pubsub_msg) = pubsub_reciever.recv() => self.handle_pubsub_msg(pubsub_msg).await,
                Some(client_msg) = self.socket.recv() => self.handle_client_msg(client_msg?).await,
                else => {
                    break;
                }
            }
        }

        Ok(())
    }
}
