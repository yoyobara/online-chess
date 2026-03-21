use std::num::ParseIntError;

use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, AsyncTypedCommands};
use rust_chess::board::Board;

use crate::{
    models::r#match::{MatchPlayers, MatchResult, MatchState},
    repositories::ephemeral_match::{
        EphemeralMatchRepository, EphemeralMatchRepositoryError, EphemeralMatchRepositoryResult,
    },
    utils::uuid::new_uuid_v4,
};

#[derive(Debug)]
pub struct RedisEphemeralMatchRepository {
    connection: MultiplexedConnection,
}

impl RedisEphemeralMatchRepository {
    pub fn new(connection: MultiplexedConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl EphemeralMatchRepository for RedisEphemeralMatchRepository {
    async fn pop_matchmaking_player(&self) -> EphemeralMatchRepositoryResult<Option<i32>> {
        let opt: Option<i32> = self
            .connection
            .clone()
            .lpop("matchmaking:waiting_players", None)
            .await?;

        Ok(opt)
    }

    async fn push_matchmaking_player(&self, player_id: i32) -> EphemeralMatchRepositoryResult<()> {
        self.connection
            .clone()
            .lpush("matchmaking:waiting_players", player_id)
            .await?;

        Ok(())
    }

    async fn register_match(
        &self,
        white_player_id: i32,
        black_player_id: i32,
        starting_board: Board,
    ) -> EphemeralMatchRepositoryResult<String> {
        let match_id = new_uuid_v4();

        let _: () = redis::pipe()
            .atomic()
            .hset_multiple(
                format!("matches:{}", &match_id),
                &[
                    ("white_player_id", white_player_id.to_string()),
                    ("black_player_id", black_player_id.to_string()),
                    ("game_board", serde_json::to_string(&starting_board)?),
                    ("move_count", 0.to_string()),
                    (
                        "match_result",
                        serde_json::to_string::<Option<MatchResult>>(&None)?,
                    ),
                ],
            )
            .sadd(format!("player:{}:matches", white_player_id), &match_id)
            .sadd(format!("player:{}:matches", black_player_id), &match_id)
            .query_async(&mut self.connection.clone())
            .await?;

        Ok(match_id)
    }

    async fn is_player_in_match(
        &self,
        player_id: i32,
        match_id: &str,
    ) -> EphemeralMatchRepositoryResult<bool> {
        self.connection
            .clone()
            .sismember(format!("player:{}:matches", player_id), match_id)
            .await
            .map_err(Into::into)
    }

    async fn get_match_state(&self, match_id: &str) -> EphemeralMatchRepositoryResult<MatchState> {
        let match_fields = self
            .connection
            .clone()
            .hmget(
                format!("matches:{}", match_id),
                &["game_board", "move_count", "match_result"],
            )
            .await?;

        Ok(MatchState {
            board: serde_json::from_str(&match_fields[0])?,
            move_count: match_fields[1].parse()?,
            match_result: serde_json::from_str(&match_fields[2])?,
        })
    }

    async fn update_match_state(
        &self,
        match_id: &str,
        new_state: &MatchState,
    ) -> EphemeralMatchRepositoryResult<()> {
        self.connection
            .clone()
            .hset_multiple(
                format!("matches:{}", match_id),
                &[
                    ("game_board", serde_json::to_string(&new_state.board)?),
                    ("move_count", new_state.move_count.to_string()),
                    (
                        "match_result",
                        serde_json::to_string::<Option<MatchResult>>(&new_state.match_result)?,
                    ),
                ],
            )
            .await?;

        Ok(())
    }

    async fn get_players(&self, match_id: &str) -> EphemeralMatchRepositoryResult<MatchPlayers> {
        let players = self
            .connection
            .clone()
            .hmget(
                format!("matches:{}", match_id),
                &["white_player_id", "black_player_id"],
            )
            .await?;

        Ok(MatchPlayers {
            white_player_id: players[0].parse::<i32>()?,
            black_player_id: players[1].parse::<i32>()?,
        })
    }
}

impl From<redis::RedisError> for EphemeralMatchRepositoryError {
    fn from(err: redis::RedisError) -> Self {
        EphemeralMatchRepositoryError::Unknown(err.into())
    }
}

impl From<serde_json::Error> for EphemeralMatchRepositoryError {
    fn from(err: serde_json::Error) -> Self {
        EphemeralMatchRepositoryError::Unknown(err.into())
    }
}

impl From<ParseIntError> for EphemeralMatchRepositoryError {
    fn from(err: ParseIntError) -> Self {
        EphemeralMatchRepositoryError::Unknown(err.into())
    }
}
