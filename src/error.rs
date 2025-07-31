pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DbError(#[from] db::Error),

    #[error(transparent)]
    RouterError(#[from] routes::Error)
}