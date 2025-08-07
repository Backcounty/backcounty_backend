use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug,thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    DotEnvError(#[from] dotenv::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    TcpListenerError(#[from] std::io::Error),

    #[error(transparent)]
    DbError(#[from] db::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::SerdeError(err)=>(StatusCode::INTERNAL_SERVER_ERROR,err.to_string()),
            Error::DotEnvError(err)=>(StatusCode::INTERNAL_SERVER_ERROR,err.to_string()),
            Error::ReqwestError(err)=>(StatusCode::INTERNAL_SERVER_ERROR,err.to_string()),
            Error::TcpListenerError(err)=>(StatusCode::INTERNAL_SERVER_ERROR,err.to_string()),
            Error::DbError(err)=>(StatusCode::INTERNAL_SERVER_ERROR,err.to_string()),
        }.into_response()
    }
}
