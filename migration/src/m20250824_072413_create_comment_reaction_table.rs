use super::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                TableCreateStatement::new()
                    .table("reaction_type")
                    .if_not_exists()
                    .col(
                        ColumnDef::new_with_type("reaction_type_id", ColumnType::SmallInteger)
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new_with_type("reaction", ColumnType::Text)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("reaction_type").to_owned())
            .await
    }
}
