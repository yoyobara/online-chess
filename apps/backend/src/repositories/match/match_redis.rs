use anyhow::anyhow;
use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, AsyncTypedCommands, RedisError};

use crate::{
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
        player_1_id: i32,
        player_2_id: i32,
    ) -> MatchRepositoryResult<String> {
        let match_id = new_uuid_v4();

        let _: () = redis::pipe()
            .atomic()
            .hset_multiple(
                format!("matches:{}", &match_id),
                &[("player1_id", player_1_id), ("player2_id", player_2_id)],
            )
            .sadd(format!("player:{}:matches", player_1_id), &match_id)
            .sadd(format!("player:{}:matches", player_2_id), &match_id)
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
}
