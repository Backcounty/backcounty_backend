use serde::Serialize;
use super::*;
pub(crate) mod blog_category;
pub(crate) mod blog_reaction;
pub(crate) mod blog_tag;
pub(crate) mod comment;
pub(crate) mod comment_reaction;
pub(crate) mod reaction_type;
pub(crate) mod taxonomies;
pub(crate) mod media;

#[derive(Debug, Serialize,Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blog")]
pub struct Model {
    #[sea_orm(primary_key)]
    blog_id: uuid::Uuid,
    author_id: uuid::Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    published_at: Option<chrono::DateTime<chrono::Utc>>,
    title: String,
    content: String,
    view_count: i32,
    like_count: i32,
    blog_status_id: i16,
}

#[derive(Debug, EnumIter)]
pub enum Relation {
    User,
    BlogStatus,
    Media,
    Comments,
    BlogCategory,
    BlogTag,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(user::Entity)
                .from(Column::AuthorId)
                .to(user::Column::UserId)
                .into(),
            Relation::BlogStatus => Entity::has_one(blog_status::Entity).into(),
            Relation::Media => Entity::has_many(media::Entity).into(),
            Relation::Comments => Entity::has_many(comment::Entity).into(),
            Relation::BlogCategory=>Entity::has_many(blog_category::Entity).into(),
            Relation::BlogTag=>Entity::has_many(blog_tag::Entity).into(),
        }
    }
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<blog_status::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BlogStatus.def()
    }
}

impl Related<taxonomies::categories::Entity> for Entity {
    fn to() -> RelationDef {
        blog_category::Relation::Category.def()
    }

    fn via() -> Option<RelationDef> {
        Some(blog_category::Relation::Blog.def().rev())
    }

}

impl Related<taxonomies::tags::Entity> for Entity {
    fn to() -> RelationDef {
        blog_tag::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(blog_tag::Relation::Blog.def().rev())
    }
}




impl ActiveModelBehavior for ActiveModel {}

pub mod blog_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "blog_status")]
    pub struct Model {
        #[sea_orm(primary_key)]
        blog_status_id: i16,
        #[sea_orm(default)]
        status: BlogStatus,
    }

    #[derive(Debug, Clone, PartialEq, Eq,EnumIter, DeriveActiveEnum)]
    #[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
    pub enum BlogStatus {
        #[sea_orm(string_value = "Unpublished")]
        UnPublished,
        #[sea_orm(string_value = "Published")]
        Published,
    }


    #[derive(Debug, EnumIter)]
    pub enum Relation {
        Blog,
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            Entity::belongs_to(blog::Entity)
                .from(Column::BlogStatusId)
                .to(blog::Column::BlogStatusId)
                .into()
        }
    }
    impl Related<blog::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Blog.def()
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

