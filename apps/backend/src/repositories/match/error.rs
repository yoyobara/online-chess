#[derive(thiserror::Error, Debug)]
pub enum MatchRepositoryError {
    #[error("something went wrong!")]
    Unknown(#[from] anyhow::Error),
}

pub type MatchRepositoryResult<T> = Result<T, MatchRepositoryError>;
