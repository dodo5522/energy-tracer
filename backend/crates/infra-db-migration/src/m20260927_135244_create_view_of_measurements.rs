use sea_orm::Statement;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "create or replace view generation.measurements_view as
                    select
                        m.id,
                        s.system,
                        l.label,
                        m.value,
                        u.unit,
                        m.remark,
                        m.measured_at
                    from generation.measurements m
                    join generation.labels l on m.label_id = l.id
                    join generation.systems s on m.system_id = s.id
                    join generation.units u on m.unit_id = u.id
                ;",
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("drop view generation.measurements_view;")
            .await?;
        Ok(())
    }
}
