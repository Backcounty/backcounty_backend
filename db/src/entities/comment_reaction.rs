use super::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "comment_reaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub comment_reaction_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub reaction_type_id: i16,
}

#[derive(Debug, EnumIter)]
pub enum Relation {
    User,
    ReactionType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(user::Entity)
                .from(Column::UserId)
                .to(user::Column::UserId)
                .into(),
            Relation::ReactionType => Entity::has_many(reaction_type::Entity).into(),
        }
    }
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
