use std::num::ParseIntError;

use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, AsyncTypedCommands};
use rust_chess::{
    board::Board,
    core::{chess_move::Move, color::Color},
};

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

        redis::pipe()
            .atomic()
            .hset_multiple(
                format!("matches:{}", &match_id),
                &[
                    ("white_player_id", white_player_id.to_string()),
                    ("black_player_id", black_player_id.to_string()),
                    ("current_turn", serde_json::to_string(&Color::White)?),
                    ("game_board", serde_json::to_string(&starting_board)?),
                    (
                        "match_result",
                        serde_json::to_string::<Option<MatchResult>>(&None)?,
                    ),
                ],
            )
            .sadd(format!("player:{}:matches", white_player_id), &match_id)
            .sadd(format!("player:{}:matches", black_player_id), &match_id)
            .query_async::<()>(&mut self.connection.clone())
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
                &["game_board", "current_turn", "match_result"],
            )
            .await?;

        Ok(MatchState {
            board: serde_json::from_str(&match_fields[0])?,
            current_turn: serde_json::from_str(&match_fields[1])?,
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
                    (
                        "match_result",
                        serde_json::to_string::<Option<MatchResult>>(&new_state.match_result)?,
                    ),
                ],
            )
            .await?;

        Ok(())
    }

    async fn push_move(&self, match_id: &str, mv: Move) -> EphemeralMatchRepositoryResult<()> {
        self.connection
            .clone()
            .rpush(
                format!("matches:{}:moves", match_id),
                serde_json::to_string(&mv)?,
            )
            .await?;

        Ok(())
    }

    async fn get_match_moves(&self, match_id: &str) -> EphemeralMatchRepositoryResult<Vec<Move>> {
        let moves: Vec<String> = self
            .connection
            .clone()
            .lrange(format!("matches:{}:moves", match_id), 0, -1)
            .await?;

        let deserialized = moves
            .into_iter()
            .map(|mv| serde_json::from_str::<Move>(&mv))
            .collect::<Result<Vec<Move>, _>>()?;

        Ok(deserialized)
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

    async fn finalize_match(
        &self,
        match_id: &str,
    ) -> EphemeralMatchRepositoryResult<(MatchPlayers, MatchState, Vec<Move>)> {
        let players = self.get_players(match_id).await?;
        let state = self.get_match_state(match_id).await?;
        let moves = self.get_match_moves(match_id).await?;

        redis::pipe()
            .atomic()
            .del(format!("matches:{}", match_id))
            .del(format!("matches:{}:moves", match_id))
            .srem(
                format!("player:{}:matches", players.white_player_id),
                match_id,
            )
            .srem(
                format!("player:{}:matches", players.black_player_id),
                match_id,
            )
            .query_async::<()>(&mut self.connection.clone())
            .await?;

        Ok((players, state, moves))
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
