use crate::{
    error_mapper::ErrorMapperTrait,
    models::{prelude::*, units::*},
};
use layer_domain::entity::UnitEntity;
use layer_use_case::interface::{GenerationError, UnitRepositoryTrait};
use sea_orm::{DatabaseTransaction, IntoActiveValue, entity::prelude::*};

pub struct UnitRepository {}

impl ErrorMapperTrait for UnitRepository {}

#[async_trait::async_trait]
impl UnitRepositoryTrait<DatabaseTransaction> for UnitRepository {
    async fn add(
        &self,
        tx: &DatabaseTransaction,
        unit: UnitEntity,
    ) -> Result<i64, GenerationError> {
        let result = Units::insert::<ActiveModel>(unit.into())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        Ok(result.last_insert_id)
    }

    async fn find(
        &self,
        tx: &DatabaseTransaction,
        unit: Option<&String>,
    ) -> Result<Vec<UnitEntity>, GenerationError> {
        if let Some(unit) = unit {
            let found = Units::find()
                .filter(Column::Unit.eq(unit))
                .one(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            if let Some(u) = found {
                Ok(vec![u.try_into()?])
            } else {
                Err(GenerationError::NotFound(unit.into()))
            }
        } else {
            let units = Units::find()
                .all(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            units
                .into_iter()
                .map(|u| Ok(u.try_into()?))
                .collect::<Result<Vec<UnitEntity>, _>>()
        }
    }

    async fn update(
        &self,
        tx: &DatabaseTransaction,
        unit: &UnitEntity,
    ) -> Result<i64, GenerationError> {
        let target = (&unit.unit).to_string();
        if let Some(found) = Units::find()
            .filter(Column::Unit.eq(&target))
            .one(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?
        {
            let result = Units::update(ActiveModel {
                id: found.id.into_active_value(),
                unit: found.unit.into_active_value(),
                remark: found.remark.into_active_value(),
                ..Default::default()
            })
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
            Ok(result.id)
        } else {
            Err(GenerationError::NotFound(target))
        }
    }

    async fn delete(&self, tx: &DatabaseTransaction, unit: String) -> Result<(), GenerationError> {
        let found = Units::find()
            .filter(Column::Unit.eq(&unit))
            .one(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;

        if let Some(u) = found {
            let result = Units::delete::<ActiveModel>(u.into())
                .exec(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            if result.rows_affected == 1 {
                Ok(())
            } else {
                Err(GenerationError::Unknown(unit))
            }
        } else {
            Err(GenerationError::NotFound(unit))
        }
    }
}
