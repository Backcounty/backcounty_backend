use super::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                TableCreateStatement::new()
                    .table("user")
                    .if_not_exists()
                    .col(
                        ColumnDef::new_with_type("user_id", ColumnType::Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new_with_type("name", ColumnType::Text).not_null())
                    .col(ColumnDef::new_with_type("first_name", ColumnType::Text).not_null())
                    .col(ColumnDef::new_with_type("last_name", ColumnType::Text).not_null())
                    .col(ColumnDef::new_with_type("email", ColumnType::Text).not_null().unique_key())
                    .col(ColumnDef::new_with_type("profile_photo", ColumnType::Text).not_null())
                    .col(ColumnDef::new_with_type("role_id", ColumnType::SmallInteger).not_null().default(1))
                    .col(
                        ColumnDef::new_with_type("created_at", ColumnType::TimestampWithTimeZone)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new_with_type("updated_at", ColumnType::TimestampWithTimeZone)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("user").to_owned())
            .await
    }
}
