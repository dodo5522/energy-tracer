use crate::iden::Label;
use sea_orm::{DbBackend, Statement};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table((Label::Schema, Label::Table))
                    .if_not_exists()
                    .col(big_integer(Label::Id).primary_key().auto_increment())
                    .col(string(Label::Label).not_null().unique_key())
                    .col(string(Label::Remark).not_null().default(""))
                    .col(
                        timestamp_with_time_zone(Label::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let table = format!("{}.{}", Label::Schema.to_string(), Label::Table.to_string());
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'ラベル (e.g. Array Voltage, ...)';",
                    table,
                    Label::Label.to_string()
                ),
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Label::Schema, Label::Table))
                    .to_owned(),
            )
            .await
    }
}
