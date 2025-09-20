use super::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "media")]
pub struct Model {
    #[sea_orm(primary_key)]
    media_id: i16,
    blog_id: uuid::Uuid,
    caption: String,
    media_url: String,
}

#[derive(Debug, EnumIter)]
pub enum Relation {
    Blog,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        Entity::belongs_to(blog::Entity)
            .from(Column::BlogId)
            .to(blog::Column::BlogId)
            .into()
    }
}

impl Related<blog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}