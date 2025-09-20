pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug,thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    DotEnvError(#[from] dotenv::Error),

    #[error("Custom Error: {0}")]
    Custom(& 'static str),

    #[error(transparent)]
    JwtEncodeError(#[from] jsonwebtoken::errors::Error),
}