pub use sea_orm_migration::prelude::*;

mod iden;
mod m20251101_000002_create_updated_at_setter;
mod m20251122_073958_create_systems_table;
mod m20251122_075428_create_units_table;
mod m20251122_080751_create_labels_table;
mod m20251122_081240_create_measurements_table;
mod schemas;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251101_000002_create_updated_at_setter::Migration),
            Box::new(m20251122_073958_create_systems_table::Migration),
            Box::new(m20251122_075428_create_units_table::Migration),
            Box::new(m20251122_080751_create_labels_table::Migration),
            Box::new(m20251122_081240_create_measurements_table::Migration),
        ]
    }
}
