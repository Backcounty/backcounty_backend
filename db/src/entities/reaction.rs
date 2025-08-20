use super::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "reaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub reaction_id:uuid::Uuid,
    pub user_id:uuid::Uuid,
    pub target_type:String,
    pub target_id:uuid::Uuid,
    pub reaction_type_id :u8
}
#[derive(Debug, EnumIter)]
pub enum Relation {
    User,
    ReactionType,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User=>Entity::belongs_to(user::Entity).from(Column::UserId).to(user::Column::UserId).into(),
            Relation::ReactionType=>Entity::has_one(reaction_type::Entity).into()
        }
    }
}

impl Related<user::Entity> for Entity{
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub mod reaction_type{
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "reaction_type")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub reaction_type_id:u8,
        pub reaction :String
    }

    #[derive(Debug, EnumIter)]
    pub enum Relation {
        Reaction,
    }

    impl RelationTrait for  Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Reaction=>Entity::belongs_to(reaction::Entity).from(Column::ReactionTypeId).to(reaction::Column::ReactionTypeId).into()
            }
        }
    }

    impl Related<reaction::Entity> for Entity{
        fn to() -> RelationDef {
            Relation::Reaction.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

