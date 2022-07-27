use gloo::storage::errors::StorageError;
use thiserror::Error;

pub type ConduitWebResult<T> = Result<T, ConduitWebError>;

#[derive(Error, Debug)]
pub enum ConduitWebError {
    #[error("stored user was not found in local storage")]
    StoredUserNotFound(#[from] StorageError),
    #[error("token not available")]
    TokenNotAvailable,
    #[error("profile not found")]
    ProfileNotFound,
    #[error("article was not created")]
    ArticleNotCreated,
    #[error("article was not found")]
    ArticleNotFound,
    #[error("comments was not loaded")]
    CommentsNotLoaded,
    #[error("Date time is an invalid format")]
    DateTimeInvalid,
}
