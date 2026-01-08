mod error;
mod r#match;
mod match_redis;

pub use error::{MatchRepositoryError, MatchRepositoryResult};
pub use match_redis::RedisMatchRepository;
pub use r#match::MatchRepository;
