use std::sync::Arc;

use sea_orm::{DatabaseConnection, Set};
use chrono::Utc;

use model::UserProfile;

use crate::Result;
use crate::entities::{
    user
};


pub(crate) struct UserRepo(Arc<DatabaseConnection>);

impl UserRepo {
    pub(crate) fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }

    pub async fn create_user(&self,user:UserProfile)->Result<()>{
        let sth=user::ActiveModel{
            user_id:Set(uuid::Uuid::new_v4()),
            name:Set(user.name),
            first_name:Set(user.first_name),
            last_name:Set(user.last_name),
            email:Set(user.email),
            profile_photo:Set(user.photo),
            created_at:Set(Utc::now()),
            updated_at:Set(Utc::now()),

        };
        Ok(())
    }
}