mod ephemeral_match;
mod ephemeral_match_redis;
mod error;

pub use ephemeral_match::EphemeralMatchRepository;
pub use ephemeral_match_redis::RedisEphemeralMatchRepository;
pub use error::{EphemeralMatchRepositoryError, EphemeralMatchRepositoryResult};
