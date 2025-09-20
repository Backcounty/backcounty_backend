use super::*;
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "reaction_type")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub reaction_type_id: i16,
    pub reaction: String,
}

#[derive(Debug, EnumIter)]
pub enum Relation {
    Blog,
    Comment,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Blog => Entity::belongs_to(blog_reaction::Entity)
                .from(Column::ReactionTypeId)
                .to(blog_reaction::Column::ReactionTypeId)
                .into(),
            Relation::Comment => Entity::belongs_to(comment_reaction::Entity)
                .from(Column::ReactionTypeId)
                .to(comment_reaction::Column::ReactionTypeId)
                .into(),
        }
    }
}

impl Related<blog_reaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Blog.def()
    }
}

impl Related<comment_reaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
