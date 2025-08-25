use super::*;
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blog_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub blog_tag_id:uuid::Uuid,
    pub blog_id:uuid::Uuid,
    pub tag_id:i16,
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Blog,
    Tag,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Blog=>Entity::belongs_to(blog::Entity).from(Column::BlogId).to(blog::Column::BlogId).into(),
            Relation::Tag=>Entity::belongs_to(taxonomies::tags::Entity).from(Column::TagId).to(taxonomies::tags::Column::TagId).into(),
        }
    }
}

impl Related<blog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl Related<taxonomies::tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
