use migrations::{admins, notices, rules};
pub use sea_orm_migration::prelude::*;

mod migrations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(rules::Migration),
            Box::new(admins::Migration),
            Box::new(notices::Migration),
        ]
    }
}
