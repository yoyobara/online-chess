#[derive(thiserror::Error, Debug)]
pub enum EphemeralMatchRepositoryError {
    #[error("something went wrong!")]
    Unknown(#[from] anyhow::Error),
}

pub type EphemeralMatchRepositoryResult<T> = Result<T, EphemeralMatchRepositoryError>;
