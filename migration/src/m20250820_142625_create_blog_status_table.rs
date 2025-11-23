// use sea_orm_migration::{prelude::*, schema::*};
use super::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                TableCreateStatement::new()
                    .table("blog_status")
                    .if_not_exists()
                    .col(ColumnDef::new_with_type("blog_status_id",ColumnType::SmallInteger).not_null().primary_key())
                    .col(ColumnDef::new_with_type("status",ColumnType::Text).not_null())
                    .to_owned(),
            )
            .await?;

        // 2️⃣ Insert initial rows
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Alias::new("blog_status"))
                    .columns([
                        Alias::new("blog_status_id"),
                        Alias::new("status"),
                    ])
                    .values_panic([0.into(), "Pending".into()])
                    .values_panic([1.into(), "Published".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table("blog_status").to_owned())
            .await
    }
}


