use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;
use model::UserProfile;

use crate::entities::user;
use crate::entities::user::Model;
use crate::Result;
use super::*;
pub struct UserRepo(Arc<DatabaseConnection>);

impl UserRepo {
    pub(crate) fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }

    pub async fn create_user(&self, user: &UserProfile) -> Result<Uuid> {
        let user_id=Uuid::new_v4();
        let user_model = user::ActiveModel {
            user_id: Set(user_id.clone()),
            name: Set(user.name.clone()),
            first_name: Set(user.first_name.clone()),
            last_name: Set(user.last_name.clone()   ),
            email: Set(user.email.clone()),
            profile_photo: Set(user.photo.clone()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };
        user_model.insert(self.0.as_ref()).await?;

        Ok(user_id)
    }

    pub async fn get_user_from_email(&self, email: &str) -> Result<Option<Model>> {
        let user=user::Entity::find()
            .filter(user::Column::Email.contains(email))
            .one(self.0.as_ref())
            .await?;

        Ok(user)
    }
}
