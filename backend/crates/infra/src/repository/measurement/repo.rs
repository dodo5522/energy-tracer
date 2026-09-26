use crate::{
    error_mapper::ErrorMapperTrait,
    models::{
        labels::{Column as LabelDbColumn, Entity as LabelDbEntity},
        measurements::{ActiveModel as MeasurementDbActiveModel, Entity as MeasurementDbEntity},
        systems::{Column as SystemDbColumn, Entity as SystemDbEntity},
        units::{Column as UnitDbColumn, Entity as UnitDbEntity},
    },
};
use chrono::{DateTime, Utc};
use layer_domain::entity::MeasurementEntity;
use layer_use_case::interface::{GenerationError, MeasurementRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseTransaction};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use std::collections::HashMap;

pub struct MeasurementRepository {}

impl ErrorMapperTrait for MeasurementRepository {}

impl MeasurementRepository {
    async fn label_to_id(
        tx: &DatabaseTransaction,
        measurements: &[MeasurementEntity],
    ) -> Result<HashMap<String, i64>, GenerationError> {
        Ok(LabelDbEntity::find()
            .select_only()
            .columns([LabelDbColumn::Label, LabelDbColumn::Id])
            .filter(LabelDbColumn::Label.is_in(measurements.iter().map(|m| &m.label)))
            .into_tuple::<(String, i64)>()
            .all(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?
            .into_iter()
            .collect::<HashMap<String, i64>>())
    }

    async fn system_to_id(
        tx: &DatabaseTransaction,
        measurements: &[MeasurementEntity],
    ) -> Result<HashMap<String, i64>, GenerationError> {
        Ok(SystemDbEntity::find()
            .select_only()
            .columns([SystemDbColumn::System, SystemDbColumn::Id])
            .filter(SystemDbColumn::System.is_in(measurements.iter().map(|m| &m.system)))
            .into_tuple::<(String, i64)>()
            .all(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?
            .into_iter()
            .collect::<HashMap<String, i64>>())
    }

    async fn unit_to_id(
        tx: &DatabaseTransaction,
        measurements: &[MeasurementEntity],
    ) -> Result<HashMap<String, i64>, GenerationError> {
        Ok(UnitDbEntity::find()
            .select_only()
            .columns([UnitDbColumn::Unit, UnitDbColumn::Id])
            .filter(UnitDbColumn::Unit.is_in(measurements.iter().map(|m| m.unit.to_string())))
            .into_tuple::<(String, i64)>()
            .all(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?
            .into_iter()
            .collect::<HashMap<String, i64>>())
    }
}

#[async_trait::async_trait]
impl MeasurementRepositoryTrait<DatabaseTransaction> for MeasurementRepository {
    async fn add(
        &self,
        tx: &DatabaseTransaction,
        measurements: Vec<MeasurementEntity>,
    ) -> Result<(), GenerationError> {
        if measurements.is_empty() {
            return Ok(());
        }

        let labels = Self::label_to_id(tx, &measurements).await?;
        let systems = Self::system_to_id(tx, &measurements).await?;
        let units = Self::unit_to_id(tx, &measurements).await?;

        let measurements = measurements
            .into_iter()
            .map(|new| {
                let target_unit = new.unit.to_string();
                Ok(MeasurementDbActiveModel {
                    label_id: ActiveValue::Set(
                        labels
                            .get(&new.label)
                            .copied()
                            .ok_or(GenerationError::NotFound(new.label))?,
                    ),
                    unit_id: ActiveValue::Set(
                        units
                            .get(&target_unit)
                            .copied()
                            .ok_or_else(|| GenerationError::NotFound(target_unit))?,
                    ),
                    system_id: ActiveValue::Set(
                        systems
                            .get(&new.system)
                            .copied()
                            .ok_or(GenerationError::NotFound(new.system))?,
                    ),
                    value: ActiveValue::Set(new.value),
                    measured_at: ActiveValue::Set(new.measured_at.into()),
                    ..Default::default()
                })
            })
            .collect::<Result<Vec<MeasurementDbActiveModel>, GenerationError>>()?;

        MeasurementDbEntity::insert_many(measurements)
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;

        Ok(())
    }

    async fn fetch(
        &self,
        tx: &DatabaseTransaction,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        system: String,
        labels: Option<Vec<String>>,
    ) -> Result<Vec<MeasurementEntity>, GenerationError> {
        // let h = Measurements::find_by_id::<i64>(id.into())
        //     .one(tx)
        //     .await
        //     .map_err(Self::map_db_to_generation_error)?;
        //
        // if let Some(measurement) = h {
        //     Ok(Some(MeasurementEntity {
        //         value: measurement.value,
        //         unit: measurement
        //             .unit
        //             .clone()
        //             .try_into()
        //             .map_err(|_| Self::map_invalid_unit(measurement.unit))?,
        //         sub_system: measurement.sub_system,
        //         label: measurement.label,
        //         monitored_at: measurement.measured_at.into(),
        //     }))
        // } else {
        //     Ok(None)
        // }
        Ok(vec![])
    }

    async fn delete(&self, tx: &DatabaseTransaction, id: i64) -> Result<(), GenerationError> {
        Err(GenerationError::NotImplemented(
            "HistoryRepository::delete()".to_string(),
        ))
    }
}
