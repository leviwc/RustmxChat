pub use sea_orm_migration::prelude::*;

mod m20240302_000001_create_user_table;
mod m20240302_000002_create_user_to_user_chat_table;
mod m20240203_191931_message;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240302_000001_create_user_table::Migration),
            Box::new(m20240302_000002_create_user_to_user_chat_table::Migration),
            Box::new(m20240203_191931_message::Migration),
        ]
    }
}
