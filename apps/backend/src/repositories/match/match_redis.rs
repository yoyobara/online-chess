use async_trait::async_trait;
use redis::aio::MultiplexedConnection;

use crate::repositories::r#match::{MatchRepository, MatchRepositoryResult};

#[derive(Debug)]
pub struct RedisMatchRepository {
    connection: MultiplexedConnection,
}

impl RedisMatchRepository {
    pub fn new(connection: MultiplexedConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl MatchRepository for RedisMatchRepository {
    async fn pop_matchmaking_player(&self) -> MatchRepositoryResult<Option<i32>> {
        todo!()
    }

    async fn push_matchmaking_player(&self, player_id: i32) -> MatchRepositoryResult<()> {
        todo!()
    }

    async fn register_match(
        &self,
        player_1_id: i32,
        player_2_id: i32,
    ) -> MatchRepositoryResult<()> {
        todo!()
    }

    async fn is_player_in_match(
        &self,
        player_id: i32,
        match_id: &str,
    ) -> MatchRepositoryResult<bool> {
        todo!()
    }
}
