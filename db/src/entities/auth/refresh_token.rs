use super::*;
#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "refresh_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub refresh_token_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub claims: serde_json::Value,
    pub token:String,
    pub revoked: bool,
    pub rotate: bool,
}

#[derive(Debug, EnumIter)]
pub enum Relation {
    Empty
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        Relation::Empty.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}
