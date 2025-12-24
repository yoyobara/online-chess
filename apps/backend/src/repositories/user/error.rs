#[derive(thiserror::Error, Debug)]
pub enum UserRepositoryError {
    #[error("user not found")]
    UserNotFound,

    #[error("violation of constraint")]
    ConstraintViolation(String),

    #[error("something went wrong!")]
    Db(#[from] anyhow::Error),
}
