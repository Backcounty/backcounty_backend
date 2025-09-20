mod m20250820_142150_create_user_table;
mod m20250820_142626_create_blog_table;
mod m20250820_142705_create_comment_table;
mod m20250820_142728_create_blog_reaction_table;
mod m20250820_150144_create_media_table;
mod m20250820_150202_create_reaction_type_table;
mod m20250820_150906_create_categories_table;
mod m20250820_150943_create_tags_table;
mod m20250821_020123_create_comment_reaction_table;
mod m20250820_142625_create_blog_status_table;
mod m20250824_072254_create_blog_category_table;
mod m20250824_072317_create_blog_tag_table;
mod m20250824_072344_create_reaction_type_table;
mod m20250824_072413_create_comment_reaction_table;
mod m20250902_155903_create_refresh_token_table;

use sea_orm_migration::MigratorTrait;
use sea_orm_migration::{
    MigrationTrait,
    manager::SchemaManager,
};
use sea_orm::{DbErr,DeriveMigrationName};
use sea_orm::sea_query::{ColumnDef, Table};
use sea_orm::sea_query::TableCreateStatement;
use sea_orm_migration::sea_query::ColumnType;
use sea_orm::sea_query::ForeignKeyCreateStatement;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250820_142150_create_user_table::Migration),
            Box::new(m20250820_142625_create_blog_status_table::Migration),
            Box::new(m20250820_142626_create_blog_table::Migration),
            Box::new(m20250820_142705_create_comment_table::Migration),
            Box::new(m20250820_150202_create_reaction_type_table::Migration),
            Box::new(m20250820_142728_create_blog_reaction_table::Migration),
            Box::new(m20250820_150144_create_media_table::Migration),
            Box::new(m20250820_150906_create_categories_table::Migration),
            Box::new(m20250820_150943_create_tags_table::Migration),
            Box::new(m20250821_020123_create_comment_reaction_table::Migration),
            Box::new(m20250824_072254_create_blog_category_table::Migration),
            Box::new(m20250824_072317_create_blog_tag_table::Migration),
            Box::new(m20250824_072344_create_reaction_type_table::Migration),
            Box::new(m20250824_072413_create_comment_reaction_table::Migration),
            Box::new(m20250902_155903_create_refresh_token_table::Migration),
        ]
    }
}
