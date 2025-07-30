use sea_orm::{ActiveModelBehavior, DeriveEntityModel,EnumIter,DeriveRelation,DerivePrimaryKey,PrimaryKeyTrait};


#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "cake")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub profile_photo: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}