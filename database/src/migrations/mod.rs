use sea_orm_migration::{MigrationTrait, MigratorTrait, async_trait};

pub mod _2026_10_15_create_users;
pub mod _2026_10_16_create_challenges;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        return vec![
            Box::new(_2026_10_15_create_users::Migration),
            Box::new(_2026_10_16_create_challenges::Migration),
        ];
    }
}
