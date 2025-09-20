use super::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                TableCreateStatement::new()
                    .table("media")
                    .if_not_exists()
                    .col(
                        ColumnDef::new_with_type("media_id", ColumnType::SmallInteger)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new_with_type("blog_id", ColumnType::Uuid).not_null())
                    .col(ColumnDef::new_with_type("media_url", ColumnType::Text).not_null())
                    .col(ColumnDef::new_with_type("caption", ColumnType::Text).not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_media_blog")
                            .from("media", "blog_id")
                            .to("blog", "blog_id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("media").to_owned()).await
    }
}