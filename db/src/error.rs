pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug,thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DotEnvError(#[from] dotenv::Error),
    
    #[error(transparent)]
    SeaormDbError(#[from] sea_orm::error::DbErr),
}
