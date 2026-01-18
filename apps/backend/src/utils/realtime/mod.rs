use axum::extract::ws::WebSocket;

use crate::{state::AppState, utils::pubsub::PubSub};

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

    pub async fn mainloop(mut self) -> anyhow::Result<()> {
        let mut pubsub_reciever = self
            .pubsub
            .subscribe(&format!("match:{}", self.match_id))
            .await?;

        Ok(())
    }
}
