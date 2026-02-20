use anyhow::anyhow;
use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, AsyncTypedCommands, RedisError};
use rust_chess::board::Board;

use crate::{
    models::r#match::{MatchPlayers, MatchState},
    repositories::r#match::{MatchRepository, MatchRepositoryError, MatchRepositoryResult},
    utils::uuid::new_uuid_v4,
};

#[derive(Debug)]
pub struct RedisMatchRepository {
    connection: MultiplexedConnection,
}

impl RedisMatchRepository {
    pub fn new(connection: MultiplexedConnection) -> Self {
        Self { connection }
    }
}

fn redis_to_repo_error(err: RedisError) -> MatchRepositoryError {
    MatchRepositoryError::Unknown(anyhow!(err))
}

#[async_trait]
impl MatchRepository for RedisMatchRepository {
    async fn pop_matchmaking_player(&self) -> MatchRepositoryResult<Option<i32>> {
        let opt: Option<i32> = self
            .connection
            .clone()
            .lpop("matchmaking:waiting_players", None)
            .await
            .map_err(redis_to_repo_error)?;

        Ok(opt)
    }

    async fn push_matchmaking_player(&self, player_id: i32) -> MatchRepositoryResult<()> {
        self.connection
            .clone()
            .lpush("matchmaking:waiting_players", player_id)
            .await
            .map_err(redis_to_repo_error)?;

        Ok(())
    }

    async fn register_match(
        &self,
        white_player_id: i32,
        black_player_id: i32,
    ) -> MatchRepositoryResult<String> {
        let match_id = new_uuid_v4();
        let new_board = Board::new();

        let _: () = redis::pipe()
            .atomic()
            .hset_multiple(
                format!("matches:{}", &match_id),
                &[
                    ("white_player_id", white_player_id.to_string()),
                    ("black_player_id", black_player_id.to_string()),
                    (
                        "game_board",
                        serde_json::to_string(&new_board).map_err(anyhow::Error::from)?,
                    ),
                    ("move_count", 0.to_string()),
                ],
            )
            .sadd(format!("player:{}:matches", white_player_id), &match_id)
            .sadd(format!("player:{}:matches", black_player_id), &match_id)
            .query_async(&mut self.connection.clone())
            .await
            .map_err(redis_to_repo_error)?;

        Ok(match_id)
    }

    async fn is_player_in_match(
        &self,
        player_id: i32,
        match_id: &str,
    ) -> MatchRepositoryResult<bool> {
        self.connection
            .clone()
            .sismember(format!("player:{}:matches", player_id), &match_id)
            .await
            .map_err(redis_to_repo_error)
    }

    async fn get_match_state(&self, match_id: &str) -> MatchRepositoryResult<MatchState> {
        let match_fields = self
            .connection
            .clone()
            .hmget(
                &format!("matches:{}", match_id),
                &["game_board", "move_count"],
            )
            .await
            .map_err(redis_to_repo_error)?;

        Ok(MatchState {
            board: serde_json::from_str(&match_fields[0]).map_err(anyhow::Error::from)?,
            move_count: match_fields[1].parse().map_err(anyhow::Error::from)?,
        })
    }

    async fn update_match_state(
        &self,
        match_id: &str,
        new_state: &MatchState,
    ) -> MatchRepositoryResult<()> {
        self.connection
            .clone()
            .hset_multiple(
                format!("matches:{}", match_id),
                &[
                    (
                        "game_board",
                        serde_json::to_string(&new_state.board).map_err(anyhow::Error::from)?,
                    ),
                    ("move_count", new_state.move_count.to_string()),
                ],
            )
            .await
            .map_err(redis_to_repo_error)?;

        Ok(())
    }

    async fn get_players(&self, match_id: &str) -> MatchRepositoryResult<MatchPlayers> {
        let players = self
            .connection
            .clone()
            .hmget(
                &format!("matches:{}", match_id),
                &["white_player_id", "black_player_id"],
            )
            .await
            .map_err(redis_to_repo_error)?;

        Ok(MatchPlayers {
            white_player_id: players[0].parse::<i32>().map_err(anyhow::Error::from)?,
            black_player_id: players[1].parse::<i32>().map_err(anyhow::Error::from)?,
        })
    }
}
