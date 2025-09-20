use super::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                TableCreateStatement::new()
                    .table("blog_category")
                    .if_not_exists()
                    .col(
                        ColumnDef::new_with_type("blog_category_id", ColumnType::SmallInteger)
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new_with_type("blog_id", ColumnType::SmallInteger)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new_with_type("category_id", ColumnType::SmallInteger)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("blog_category").to_owned())
            .await
    }
}
