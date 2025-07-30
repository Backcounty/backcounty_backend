use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::Result;
use crate::entities::{
    user
};

pub(crate) struct UserRepo(Arc<DatabaseConnection>);

impl UserRepo {
    pub(crate) fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }

    pub async fn create_user(&self,)->Result<()>{
        let sth=user::ActiveModel{

        };
        Ok(())
    }
}