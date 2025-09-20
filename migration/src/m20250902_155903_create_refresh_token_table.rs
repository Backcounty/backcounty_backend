use super::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                TableCreateStatement::new()
                    .table("refresh_token")
                    .if_not_exists()
                    .col(
                        ColumnDef::new_with_type("refresh_token_id", ColumnType::Uuid)
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new_with_type("session_id", ColumnType::Uuid).not_null())
                    .col(ColumnDef::new_with_type("created_at", ColumnType::TimestampWithTimeZone).not_null())
                    .col(ColumnDef::new_with_type("updated_at", ColumnType::TimestampWithTimeZone).not_null())
                    .col(ColumnDef::new_with_type("claims", ColumnType::Json).not_null())
                    .col(ColumnDef::new_with_type("token", ColumnType::Text).unique_key().not_null())
                    .col(
                        ColumnDef::new_with_type("revoked", ColumnType::Boolean)
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new_with_type("rotate", ColumnType::Boolean)
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("refresh_token").to_owned())
            .await
    }
}
