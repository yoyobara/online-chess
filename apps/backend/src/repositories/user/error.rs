#[derive(thiserror::Error, Debug)]
pub enum UserRepositoryError {
    #[error("user not found")]
    UserNotFound,

    #[error("{0} is already taken")]
    ConstraintViolation(String),

    #[error("something went wrong!")]
    Db(#[from] anyhow::Error),
}
