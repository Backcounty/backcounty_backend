use super::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                TableCreateStatement::new()
                    .table("blog_tag")
                    .if_not_exists()
                    .col(
                        ColumnDef::new_with_type("blog_tag_id", ColumnType::SmallInteger)
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new_with_type("blog_id", ColumnType::SmallInteger)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new_with_type("tag_id", ColumnType::SmallInteger)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("blog_tag").to_owned())
            .await
    }
}
