use super::*;

#[derive(Debug,Clone,PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blog")]
pub struct Model {
    #[sea_orm(primary_key)]
    blog_id:uuid::Uuid,
    user_id:uuid::Uuid,
    created_at:chrono::DateTime<chrono::Utc>,
    updated_at:chrono::DateTime<chrono::Utc>,
    published_at:chrono::DateTime<chrono::Utc>,
    title:String,
    content:String,
    view_count:u16,
    like_count:u16,
    blog_status_id:u8,
}

#[derive(Debug,EnumIter)]
pub enum Relation {
    User,
    BlogStatus,
    Media,
    Comments,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User=>Entity::belongs_to(user::Entity).from(Column::UserId).to(user::Column::UserId).into(),
            Relation::BlogStatus=>Entity::has_one(blog_status::Entity).into(),
            Relation::Media=>Entity::has_many(media::Entity).into(),
            Relation::Comments=>Entity::has_many(comment::Entity).into(),
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

impl ActiveModelBehavior for ActiveModel {}

pub mod blog_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "blog_status")]
    pub struct Model {
        #[sea_orm(primary_key)]
        blog_status_id:u8,
        status:String,
    }

    #[derive(Debug,EnumIter)]
    pub enum Relation {
        Blog
    }

    impl RelationTrait for Relation{
        fn def(&self) -> RelationDef {
            Entity::belongs_to(blog::Entity).from(Column::BlogStatusId).to(blog::Column::BlogStatusId).into()
        }
    }
    impl Related<blog::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Blog.def()
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod media{
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "media")]
    pub struct Model {
        #[sea_orm(primary_key)]
        media_id:u8,
        blog_id:u8,
        caption:String,
        media_url:String,
        status:String,
    }

    #[derive(Debug,EnumIter)]
    pub enum Relation {
        Blog
    }

    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            Entity::belongs_to(blog::Entity).from(Column::BlogId).to(blog::Column::BlogId).into()
        }
    }

    impl Related<blog::Entity> for  Entity {
        fn to() -> RelationDef {
            Relation::Blog.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

