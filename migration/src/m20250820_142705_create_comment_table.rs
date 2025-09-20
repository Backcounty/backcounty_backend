use super::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                TableCreateStatement::new()
                    .table("comment")
                    .if_not_exists()
                    .col(
                        ColumnDef::new_with_type("comment_id", ColumnType::Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new_with_type("blog_id", ColumnType::Uuid).not_null())
                    .col(ColumnDef::new_with_type("user_id", ColumnType::Uuid).not_null())
                    .col(ColumnDef::new_with_type("parent_comment_id", ColumnType::Uuid))
                    .col(ColumnDef::new_with_type("content", ColumnType::Text).not_null())
                    .col(
                        ColumnDef::new_with_type("created_at", ColumnType::TimestampWithTimeZone)
                            .not_null()
                            .date_time(),
                    )
                    .col(
                        ColumnDef::new_with_type("edited_at", ColumnType::TimestampWithTimeZone)
                            .not_null()
                            .date_time(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_comment_blog")
                            .from("comment", "blog_id")
                            .to("blog", "blog_id"),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_comment_user")
                            .from("comment", "user_id")
                            .to("user", "user_id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table("comment").to_owned()).await
    }
}