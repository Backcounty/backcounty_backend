use super::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                TableCreateStatement::new()
                    .table("tag")
                    .if_not_exists()
                    .col(
                        ColumnDef::new_with_type("tag_id", ColumnType::Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new_with_type("name", ColumnType::Text).not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("tag").to_owned()).await
    }
}