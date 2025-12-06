use axum::{extract::ws::WebSocket, http::status};
use sqlx::{Pool, Postgres};
use tokio::sync::broadcast::Receiver;

use crate::{
    internal_broadcast::{InternalMessage, InternalMessageWithReciever},
    routes::ws::{
        communicator::{Event, SessionCommunicator},
        message::{ClientMessage, ServerMessage},
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

    async fn handle_client_message(&mut self, msg: ClientMessage) {
        match msg {
            ClientMessage::LookingForMatch => self.handle_looking_for_match().await,
        }
    }

    async fn handle_internal_message(&mut self, msg: InternalMessage) {
        match msg {
            InternalMessage::MatchFound {
                match_id,
                opponent_id,
            } => self.handle_match_found(match_id, opponent_id).await,
        }
    }

    async fn handle_looking_for_match(&mut self) {
        assert_eq!(self.state, SessionState::Connected);

        let matchmaking_attempt = sqlx::query!(
            r#"
            WITH claimed AS (
                SELECT id
                FROM matches
                WHERE match_status = 'Matchmaking'
                AND player2_id IS NULL
                ORDER BY id
                LIMIT 1
                FOR UPDATE
            )
            UPDATE matches
            SET player2_id = $1,
                match_status = 'Ongoing'
            WHERE id IN (SELECT id FROM claimed)
            RETURNING id, player1_id;
            "#,
            self.player_id,
        )
        .fetch_optional(&self.pool)
        .await
        .unwrap();

        if let Some(matchmaking) = matchmaking_attempt {
            self.communicator
                .internal_notify(
                    matchmaking.player1_id,
                    InternalMessage::MatchFound {
                        match_id: matchmaking.id,
                        opponent_id: self.player_id,
                    },
                )
                .await;

            self.state = SessionState::OpponentTurn {
                match_id: matchmaking.id,
                opponent_id: matchmaking.player1_id,
            };

            self.client_log("found match instantly! opponent should play")
                .await;
        } else {
            let created_match_id = sqlx::query_scalar!(
                "INSERT INTO matches (player1_id, match_status) VALUES($1, 'Matchmaking') RETURNING id;",
                self.player_id
            )
            .fetch_one(&self.pool)
            .await
            .unwrap();

            self.state = SessionState::WaitingForMatch {
                expected_match_id: created_match_id,
            };

            self.client_log("created a match, now waiting...").await;
        }
    }

    async fn handle_match_found(&mut self, match_id: i32, opponent_id: i32) {
        assert_eq!(
            self.state,
            SessionState::WaitingForMatch {
                expected_match_id: match_id
            }
        );

        self.state = SessionState::YourTurn {
            match_id,
            opponent_id,
        };

        self.client_log("finally found match! you should play.")
            .await;
    }

    async fn client_log(&mut self, message: &'static str) {
        self.communicator.ws_send(ServerMessage::Log(message)).await;
    }
}
