#[derive(thiserror::Error, Debug)]
pub enum EphemeralMatchRepositoryError {
    #[error("something went wrong: {0}")]
    Unknown(#[from] anyhow::Error),
}

pub type EphemeralMatchRepositoryResult<T> = Result<T, EphemeralMatchRepositoryError>;
