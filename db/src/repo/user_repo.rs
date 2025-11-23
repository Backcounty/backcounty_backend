use std::sync::Arc;

use chrono::Utc;
use model::UserProfile;
use uuid::Uuid;

use super::*;
use crate::entities::user;
use crate::entities::user::Model;
use crate::{Error,Result};
pub struct UserRepo(Arc<DatabaseConnection>);

impl UserRepo {
    pub(crate) fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }

    pub async fn create_user(&self, user: &UserProfile) -> Result<Uuid> {
        let existing_user = self.get_user_from_email(&user.email).await?;
        match existing_user {
            Some(already_a_user) => Ok(already_a_user.user_id),
            None => {
                let user_model = user::ActiveModel {
                    user_id: Set(user.user_id.clone()),
                    name: Set(user.name.clone()),
                    first_name: Set(user.first_name.clone()),
                    last_name: Set(user.last_name.clone()),
                    email: Set(user.email.clone()),
                    profile_photo: Set(user.photo.clone()),
                    created_at: Set(Utc::now()),
                    updated_at: Set(Utc::now()),
                    ..Default::default()
                };
                user_model.insert(self.0.as_ref()).await?;
                Ok(user.user_id)
            }
        }
    }

    async fn get_user_from_email(&self, email: &str) -> Result<Option<Model>> {
        let user = user::Entity::find()
            .filter(user::Column::Email.contains(email))
            .one(self.0.as_ref())
            .await?;

        Ok(user)
    }

    pub async fn get_user_by_user_id(&self, user_id: &Uuid) -> Result<Model> {
        let user = user::Entity::find_by_id(user_id.clone())
            .one(self.0.as_ref())
            .await?
            .ok_or(Error::Custom("Profile Doesnt Exists."))?;
        
        Ok(user)
    }
}
