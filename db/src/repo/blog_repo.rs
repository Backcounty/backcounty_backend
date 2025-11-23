use std::sync::Arc;

use chrono::Utc;
use sea_orm::query::Condition;
use sea_orm::ColumnTrait;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use uuid::Uuid;

use crate::entities::blog::ActiveModel as BlogActiveModel;
use crate::entities::blog::Column as BlogColumn;
use crate::entities::blog::Entity as BlogEntity;
use crate::entities::blog::Model as BlogModel;
use crate::error::{Error, Result};

pub struct BlogRepo(Arc<DatabaseConnection>);
impl BlogRepo {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self(db)
    }
    pub async fn create_blog(
        &self,
        author: &Uuid,
        title: &String,
        content: &String,
    ) -> Result<Uuid> {
        let blog_id = Uuid::new_v4();
        let blog_active_model = BlogActiveModel {
            blog_id: Set(blog_id),
            author_id: Set(author.clone()),
            title: Set(title.clone()),
            content: Set(content.clone()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            blog_status_id: Set(0), // here 0 reference to unpublished
            ..Default::default()
        };
        blog_active_model.insert(self.0.as_ref()).await?;

        Ok(blog_id)
    }

    pub async fn blogs(&self) -> Result<Vec<BlogModel>> {
        let blogs = BlogEntity::find().all(self.0.as_ref()).await?;

        Ok(blogs)
    }

    pub async fn get_blog_by_blog_id(&self, blog_id: &Uuid) -> Result<BlogModel> {
        let blog_model = BlogEntity::find()
            .one(self.0.as_ref())
            .await?
            .ok_or(Error::Custom("Blog not found"))?;

        Ok(blog_model)
    }

    pub async fn list_author_blog_headers(
        &self,
        author: &Uuid,
        is_published: bool,
    ) -> Result<Vec<serde_json::Value>> {

        let mut columns_select = vec![BlogColumn::AuthorId, BlogColumn::BlogId];

        let (column_selected, blog_status_id) = if is_published {
            (BlogColumn::PublishedAt,1)
        } else {
            (BlogColumn::CreatedAt,0)
        };
        columns_select.push(column_selected);

        let condition = Condition::all()
            .add(BlogColumn::BlogStatusId.eq(blog_status_id))
            .add(BlogColumn::AuthorId.eq(author.clone()));

        let titles = BlogEntity::find()
            .select_only()
            .columns(columns_select)
            .filter(condition)
            .order_by_desc(column_selected)
            .into_json()
            .all(self.0.as_ref())
            .await?;

        Ok(titles)
    }
}
