use super::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "comment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub comment_id:uuid::Uuid,
    pub content:String,
    pub blog_id:uuid::Uuid,
    pub user_id:uuid::Uuid,
    pub parent_id:Option<uuid::Uuid>,
    pub created_at:chrono::DateTime<chrono::Utc>,
    pub edited_at:chrono::DateTime<chrono::Utc>,
}

#[derive(Debug,EnumIter)]
pub enum Relation {
    User,
    Blog,
}

impl RelationTrait for Relation{
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(user::Entity).from(Column::UserId).to(user::Column::UserId).into(),
            Self::Blog => Entity::belongs_to(blog::Entity).from(Column::BlogId).to(blog::Column::BlogId).into()
        }
    }
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<blog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
