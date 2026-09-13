use crate::{
    error_mapper::ErrorMapperTrait,
    models::{prelude::*, systems::*},
};
use layer_domain::entity::SystemEntity;
use layer_use_case::interface::{GenerationError, SubSystemRepositoryTrait};
use sea_orm::{DatabaseTransaction, entity::prelude::*};

pub struct SubSystemRepository {}

impl ErrorMapperTrait for SubSystemRepository {}

#[async_trait::async_trait]
impl SubSystemRepositoryTrait<DatabaseTransaction> for SubSystemRepository {
    async fn add(
        &self,
        tx: &DatabaseTransaction,
        system: SystemEntity,
    ) -> Result<i64, GenerationError> {
        let system: ActiveModel = system.into();
        let result = Systems::insert(system)
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        Ok(result.last_insert_id)
    }

    async fn find(
        &self,
        tx: &DatabaseTransaction,
        system: Option<&String>,
    ) -> Result<Vec<SystemEntity>, GenerationError> {
        if let Some(system) = system {
            let found = Systems::find()
                .filter(Column::System.eq(system))
                .one(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            if let Some(s) = found {
                Ok(vec![s.into()])
            } else {
                Err(GenerationError::NotFound(system.into()))
            }
        } else {
            let founds = Systems::find()
                .all(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            let systems = founds
                .into_iter()
                .map(|g| Ok(g.into()))
                .collect::<Result<Vec<SystemEntity>, GenerationError>>()?;
            Ok(systems)
        }
    }

    async fn update(
        &self,
        tx: &DatabaseTransaction,
        system: &SystemEntity,
    ) -> Result<i64, GenerationError> {
        let result = Systems::update::<ActiveModel>(system.into())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        Ok(result.id)
    }

    async fn delete(
        &self,
        tx: &DatabaseTransaction,
        system: String,
    ) -> Result<(), GenerationError> {
        let found = Systems::find()
            .filter(Column::System.eq(&system))
            .one(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;

        if let Some(s) = found {
            let result = Systems::delete::<ActiveModel>(s.into())
                .exec(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            if result.rows_affected == 1 {
                Ok(())
            } else {
                Err(GenerationError::Unknown(system))
            }
        } else {
            Err(GenerationError::NotFound(system))
        }
    }
}
