use super::*;
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blog_category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub blog_category_id: i16,
    pub blog_id: uuid::Uuid,
    pub category_id: uuid::Uuid,
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Blog,
    Category,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Blog => Entity::belongs_to(blog::Entity)
                .from(Column::BlogId)
                .to(blog::Column::BlogId)
                .into(),
            Relation::Category => Entity::belongs_to(taxonomies::categories::Entity)
                .from(Column::CategoryId)
                .to(taxonomies::categories::Column::CategoryId)
                .into(),
        }
    }
}
impl Related<blog::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl Related<taxonomies::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
