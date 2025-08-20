use sea_orm::RelationDef;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: uuid::Uuid,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub profile_photo: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub role_id:u8
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Blog,
    Comments,
    Reactions,
}

impl RelationTrait for Relation{
    fn def(&self) -> RelationDef {
        match self {
            Relation::Blog=>Entity::has_many(blog::Entity).into(),
            Relation::Comments=>Entity::has_many(comment::Entity).into(),
            Relation::Reactions=>Entity::has_many(reaction::Entity).into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
