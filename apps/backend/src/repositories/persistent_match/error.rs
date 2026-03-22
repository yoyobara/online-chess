#[derive(thiserror::Error, Debug)]
pub enum PersistentMatchRepositoryError {
    #[error("something went wrong!")]
    Unknown(#[from] anyhow::Error),
}

pub type PersistentMatchRepositoryResult<T> = Result<T, PersistentMatchRepositoryError>;
