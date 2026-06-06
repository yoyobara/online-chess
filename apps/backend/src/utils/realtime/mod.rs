pub mod client_communication;

mod client_handlers;
mod pubsub_handlers;

use anyhow::anyhow;
use rust_chess::core::color::Color;
use tokio::select;
use tracing::{error, info};

use crate::{
    state::AppState,
    utils::{
        pubsub::{message::PubSubMessage, PubSub},
        realtime::{
            client_communication::{
                message::{ClientMessage, ServerMessage},
                ClientCommunicator,
            },
            client_handlers::{handle_client_chat, handle_client_join, handle_client_player_move},
        },
    },
};

pub struct RealtimeSession {
    pub app_state: AppState,
    pub communicator: Box<dyn ClientCommunicator>,
    pub pubsub: Box<dyn PubSub>,

    pub match_id: String,
    pub player_id: i32,
    pub player_color: Color,

    pub opponent_id: i32,
}

impl RealtimeSession {
    pub async fn new(
        app_state: AppState,
        communicator: Box<dyn ClientCommunicator>,
        player_id: i32,
        match_id: String,
    ) -> anyhow::Result<Self> {
        let pubsub = (app_state.pubsub_factory)();

        let players = app_state
            .ephemeral_match_repo
            .get_players(&match_id)
            .await?;

        let (player_color, opponent_id) = if player_id == players.white_player_id {
            (Color::White, players.black_player_id)
        } else {
            (Color::Black, players.white_player_id)
        };

        Ok(Self {
            app_state,
            communicator,
            pubsub,
            match_id,
            player_id,
            player_color,
            opponent_id,
        })
    }

    async fn handle_pubsub_msg(&mut self, msg: PubSubMessage) -> anyhow::Result<()> {
        match msg {
            PubSubMessage::PlayerMove(mv, new_state) => {
                self.communicator
                    .send(ServerMessage::PlayerMove(mv.into(), new_state))
                    .await
            }
            PubSubMessage::ChatMessage(chat_msg) => {
                self.communicator
                    .send(ServerMessage::ChatMessage(chat_msg))
                    .await
            }
            _ => {
                error!("Received unexpected pubsub message: {:?}", msg);
                Err(anyhow!("bad pubsub message"))
            }
        }
    }

    async fn handle_client_msg(&mut self, msg: ClientMessage) -> anyhow::Result<()> {
        match msg {
            ClientMessage::JoinGame => handle_client_join(self).await,
            ClientMessage::PlayerMove(mv) => handle_client_player_move(self, mv).await,
            ClientMessage::ChatMessage(content) => handle_client_chat(self, content).await,
        }
    }

    pub async fn mainloop(mut self) -> anyhow::Result<()> {
        let mut pubsub_reciever = self
            .pubsub
            .subscribe(&format!("match:{}", self.match_id))
            .await?;

        info!(
            "Starting realtime mainloop for player {} in match {}",
            self.player_id, self.match_id
        );

        loop {
            select! {
                maybe_pubsub = pubsub_reciever.recv() => {
                    match maybe_pubsub {
                        Some(Ok(msg)) => self.handle_pubsub_msg(msg).await?,
                        Some(Err(e)) => {
                            error!("Pubsub receiver error: {}", e);
                            break;
                        }
                        None => break,
                    }
                }
                maybe_client = self.communicator.recv() => {
                    match maybe_client {
                        Some(Ok(msg)) => self.handle_client_msg(msg).await?,
                        Some(Err(e)) => {
                            error!("Client communicator error: {}", e);
                            break;
                        }
                        None => break,
                    }
                }
            }
        }

        info!(
            "Realtime mainloop ended for player {} in match {}",
            self.player_id, self.match_id
        );
        Ok(())
    }
}
