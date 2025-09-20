use super::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                TableCreateStatement::new()
                    .table("comment_reaction")
                    .if_not_exists()
                    .col(
                        ColumnDef::new_with_type("comment_reaction_id", ColumnType::Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new_with_type("user_id", ColumnType::Uuid).not_null())
                    .col(
                        ColumnDef::new_with_type("reaction_type_id", ColumnType::SmallInteger)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_reaction_user")
                            .from("comment_reaction", "user_id")
                            .to("user", "user_id"),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_reaction_type")
                            .from("comment_reaction", "reaction_type_id")
                            .to("reaction_type", "reaction_type_id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("comment_reaction").to_owned()).await
    }
}