use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use crate::Error::Custom;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug,thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    TcpListenerError(#[from] std::io::Error),

    #[error(transparent)]
    DbError(#[from] db::Error),

    #[error("Custom Error: {0}")]
    Custom(String),

    #[error("Auth Service Error: {0}")]
    AuthServiceError(#[from] auth_service::Error),

    #[error("Http Header Value Error: {0}")]
    InvalidHeaderValue(#[from] axum::http::header::InvalidHeaderValue),

    #[error("Offset Error: {0}")]
    OffsetTimeError(#[from] cookie::time::error::ComponentRange),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Custom(msg)=>(StatusCode::INTERNAL_SERVER_ERROR,msg),
            _=>(StatusCode::INTERNAL_SERVER_ERROR,self.to_string())
        }.into_response()
    }
}
