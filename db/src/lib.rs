mod entities;
mod error;
mod migration;

use std::sync::Arc;
mod repo;

use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

pub use crate::error::{Error, Result};
use crate::migration::Migrator;
use crate::repo::UserRepo;

pub struct Db {
    pub user_repo: UserRepo,
}

impl Db {
    pub async fn init() -> Result<Self> {
        let connection = Arc::new(Database::connect(dotenv::var("DATABASE_URL")?).await?);
        Migrator::up(connection.as_ref(), None).await?;

        Ok(Self {
            user_repo: UserRepo::new(Arc::clone(&connection)),
        })
    }
}
