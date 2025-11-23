use super::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                TableCreateStatement::new()
                    .table("blog")
                    .if_not_exists()
                    .col(
                        ColumnDef::new_with_type("blog_id", ColumnType::Uuid)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new_with_type("author_id", ColumnType::Uuid).not_null())
                    .col(
                        ColumnDef::new_with_type("created_at", ColumnType::TimestampWithTimeZone)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new_with_type("updated_at", ColumnType::TimestampWithTimeZone)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new_with_type("published_at", ColumnType::TimestampWithTimeZone)
                            .null(),
                    )
                    .col(ColumnDef::new_with_type("title", ColumnType::Text).not_null())
                    .col(ColumnDef::new_with_type("content", ColumnType::Text).null())
                    .col(
                        ColumnDef::new_with_type("view_count", ColumnType::Integer)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new_with_type("like_count", ColumnType::Integer)
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new_with_type("blog_status_id", ColumnType::SmallInteger).not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_blog_user")
                            .from("blog", "author_id")
                            .to("user", "user_id"),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_blog_blog_status")
                            .from("blog", "blog_status_id")
                            .to("blog_status", "blog_status_id"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("blog").to_owned())
            .await
    }
}
