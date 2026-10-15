use sea_orm::{DatabaseBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_raw(Statement::from_string(
                DatabaseBackend::Postgres,
                r#"
                CREATE OR REPLACE FUNCTION public.set_updated_at()
                    RETURNS TRIGGER AS $$
                BEGIN
                    NEW.updated_at = CURRENT_TIMESTAMP;
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql;
                "#,
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_raw(Statement::from_string(
                DatabaseBackend::Postgres,
                "DROP FUNCTION IF EXISTS public.set_updated_at();",
            ))
            .await?;

        Ok(())
    }
}
