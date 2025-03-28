use migrations::{
    admins, coupons, image_classes, images, notices, roles, rules, skus, user_levels, users,
};
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
            Box::new(roles::Migration),
            Box::new(users::Migration),
            Box::new(skus::Migration),
            Box::new(coupons::Migration),
            Box::new(user_levels::Migration),
            Box::new(images::Migration),
            Box::new(image_classes::Migration),
        ]
    }
}
