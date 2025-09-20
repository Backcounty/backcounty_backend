use std::sync::Arc;

use chrono::Utc;
use sea_orm::IntoActiveModel;
use uuid::Uuid;

use auth_service::RefreshToken;

use super::*;
use crate::entities::refresh_token;
use crate::{Error, Result};
pub struct RefreshTokenRepo(Arc<DatabaseConnection>);

impl RefreshTokenRepo {
    pub(crate) fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }

    pub async fn create_refresh_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<()> {
        let refresh_token_model = self
            .get_refresh_token_by_token(refresh_token.0.clone())
            .await?;

        if let Some(refresh_token_model) = refresh_token_model {
            let mut active_refresh_token_model = refresh_token_model.into_active_model();
            active_refresh_token_model.updated_at=Set(Utc::now());
            active_refresh_token_model.rotate=Set(true);
            active_refresh_token_model.token=Set(refresh_token.0.clone());
            active_refresh_token_model.claims=Set(serde_json::to_value(&refresh_token.1)?);
            active_refresh_token_model.update(self.0.as_ref()).await?;
            
            return Ok(());
        }

        let refresh_token_model = refresh_token::ActiveModel {
            refresh_token_id: Set(refresh_token.1.jti.clone()),
            session_id: Set(refresh_token.1.session_id.clone()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            token: Set(refresh_token.0.clone()),
            claims: Set(serde_json::to_value(&refresh_token.1)?),
            ..Default::default()
        };
        refresh_token_model.insert(self.0.as_ref()).await?;

        Ok(())
    }

    pub async fn get_refresh_token(&self, refresh_token_id: Uuid) -> Result<String> {
        let token = refresh_token::Entity::find_by_id(refresh_token_id)
            .one(self.0.as_ref())
            .await?
            .ok_or(Error::Custom("Refresh Token Model Empty"))?
            .token;
        Ok(token)
    }

    async fn get_refresh_token_by_token(
        &self,
        token: String,
    ) -> Result<Option<refresh_token::Model>> {
        let refresh_token = refresh_token::Entity::find()
            .filter(refresh_token::Column::Token.eq(&token))
            .one(self.0.as_ref())
            .await?;

        Ok(refresh_token)
    }

    pub async fn update_refresh_token<F>(
        &self,
        refresh_token: RefreshToken,
        updater: F,
    ) -> Result<()>
    where
        F: FnOnce(&mut refresh_token::ActiveModel),
    {
        let mut token_active_model = self
            .get_refresh_token_by_token(refresh_token.0)
            .await?
            .ok_or(Error::Custom("Refresh Token Model Empty"))?
            .into_active_model();

        updater(&mut token_active_model);
        token_active_model.update(self.0.as_ref()).await?;

        Ok(())
    }
}
