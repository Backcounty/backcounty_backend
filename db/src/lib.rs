mod entities;
mod error;
mod migration;

use std::sync::Arc;
mod repo;

use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use crate::migration::Migrator;
use crate::Result;
use crate::error::*;
use crate::repo::UserRepo;

pub struct Db{
    file_repo: UserRepo
}

impl Db {
    async fn init() -> Result<Self> {
        let connection = Arc::new(Database::connect(dotenv::var("DATABASE_URL")?).await?);
        Migrator::up(connection.as_ref(), None).await?;

        Ok(Self{
            file_repo: UserRepo::new(Arc::clone(&connection))
        })
    }

}
