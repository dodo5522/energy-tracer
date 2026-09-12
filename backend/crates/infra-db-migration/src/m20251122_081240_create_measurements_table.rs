use crate::iden::{Label, Measurement, System, Unit};
use crate::sea_orm::{DbBackend, Statement};
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{big_integer, float, string, timestamp_with_time_zone};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = format!(
            "{}.{}",
            Measurement::Schema.to_string(),
            Measurement::Table.to_string()
        );
        manager
            .create_table(
                Table::create()
                    .table((Measurement::Schema, Measurement::Table))
                    .if_not_exists()
                    .col(big_integer(Measurement::Id).primary_key().auto_increment())
                    .col(string(Measurement::SubSystem).not_null())
                    .col(string(Measurement::Label).not_null())
                    .col(string(Measurement::Unit).not_null())
                    .col(float(Measurement::Value).not_null())
                    .col(string(Measurement::Remark).not_null().default(""))
                    .col(
                        timestamp_with_time_zone(Measurement::MeasuredAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Measurement::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Measurement::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-measurements-group")
                            .from(
                                (Measurement::Schema, Measurement::Table),
                                Measurement::SubSystem,
                            )
                            .to((System::Schema, System::Table), System::System)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-measurements-label")
                            .from(
                                (Measurement::Schema, Measurement::Table),
                                Measurement::Label,
                            )
                            .to((Label::Schema, Label::Table), Label::Label)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-measurements-unit")
                            .from((Measurement::Schema, Measurement::Table), Measurement::Unit)
                            .to((Unit::Schema, Unit::Table), Unit::Unit)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'グループ';",
                    table,
                    Measurement::SubSystem.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS 'ラベル';",
                    table,
                    Measurement::Label.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '単位';",
                    table,
                    Measurement::Unit.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '値';",
                    table,
                    Measurement::Value.to_string()
                ),
            ))
            .await?;
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "COMMENT ON COLUMN {}.{} IS '観測日時';",
                    table,
                    Measurement::MeasuredAt.to_string()
                ),
            ))
            .await?;

        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                r#"
                CREATE TRIGGER updated_at_setter
                BEFORE UPDATE ON generation.measurements
                FOR EACH ROW
                EXECUTE FUNCTION public.set_updated_at();
                "#,
            ))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table((Measurement::Schema, Measurement::Table))
                    .to_owned(),
            )
            .await
    }
}
