use crate::iden::System;
use crate::sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{big_integer, string, timestamp_with_time_zone};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = format!(
            "{}.{}",
            System::Schema.to_string(),
            System::Table.to_string()
        );
        manager
            .create_table(
                Table::create()
                    .table((System::Schema, System::Table))
                    .if_not_exists()
                    .col(big_integer(System::Id).primary_key().auto_increment())
                    .col(string(System::System).not_null().unique_key())
                    .col(string(System::Remark).not_null().default(""))
                    .col(
                        timestamp_with_time_zone(System::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'システム (e.g. Array, Battery, ...)';",
                    table,
                    System::System.to_string()
                ),
            ))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((System::Schema, System::Table))
                    .to_owned(),
            )
            .await
    }
}
