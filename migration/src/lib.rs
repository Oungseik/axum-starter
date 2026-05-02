pub use sea_orm_migration::prelude::*;

mod m20260502_000001_create_user_table;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260502_000001_create_user_table::Migration)]
    }
}
