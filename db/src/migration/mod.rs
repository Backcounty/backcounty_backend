mod m20250820_142150_create_user_table;
mod m20250820_142626_create_blog_table;
mod m20250820_142705_create_comment_table;
mod m20250820_142728_create_reaction_table;
mod m20250820_142801_create_taxonomies_table;

use sea_orm_migration::MigratorTrait;

use sea_orm_migration::{
    MigrationTrait,
    manager::SchemaManager,
};
use sea_orm::{DbErr,DeriveMigrationName};
use sea_orm::sea_query::{ColumnDef, Table};
use sea_orm::sea_query::TableCreateStatement;
use sea_orm_migration::sea_query::ColumnType;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250820_142150_create_user_table::Migration),
            Box::new(m20250820_142626_create_blog_table::Migration),
            Box::new(m20250820_142705_create_comment_table::Migration),
            Box::new(m20250820_142728_create_reaction_table::Migration),
            Box::new(m20250820_142801_create_taxonomies_table::Migration),
        ]
    }
}
