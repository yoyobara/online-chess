#[derive(thiserror::Error, Debug)]
pub enum PersistentMatchRepositoryError {
    #[error("something went wrong: {0}")]
    Unknown(#[from] anyhow::Error),
}

pub type PersistentMatchRepositoryResult<T> = Result<T, PersistentMatchRepositoryError>;
