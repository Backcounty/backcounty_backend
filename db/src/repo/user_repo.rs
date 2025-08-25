use std::sync::Arc;

use chrono::Utc;

use model::UserProfile;

use crate::entities::user;
use crate::Result;
use super::*;
pub struct UserRepo(Arc<DatabaseConnection>);

impl UserRepo {
    pub(crate) fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }

    pub async fn create_user(&self, user: UserProfile) -> Result<()> {
        let user_model = user::ActiveModel {
            user_id: Set(uuid::Uuid::new_v4()),
            name: Set(user.name),
            first_name: Set(user.first_name),
            last_name: Set(user.last_name),
            email: Set(user.email),
            profile_photo: Set(user.photo),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        user_model.insert(self.0.as_ref()).await?;

        Ok(())
    }

    pub async fn user_is_present(&self, email: &str) -> Result<bool> {
        let is_present=user::Entity::find()
            .filter(user::Column::Email.contains(email))
            .one(self.0.as_ref())
            .await?
            .is_some();

        Ok(is_present)
    }
}
