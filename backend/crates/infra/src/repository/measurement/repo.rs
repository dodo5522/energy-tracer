use crate::{
    error_mapper::ErrorMapperTrait,
    models::{measurements::ActiveModel, prelude::Measurements},
};
use chrono::{DateTime, Utc};
use layer_domain::entity::MeasurementEntity;
use layer_use_case::interface::{GenerationError, MeasurementRepositoryTrait};
use sea_orm::{ActiveValue, DatabaseTransaction, entity::EntityTrait};

pub struct MeasurementRepository {}

impl ErrorMapperTrait for MeasurementRepository {}

#[async_trait::async_trait]
impl MeasurementRepositoryTrait<DatabaseTransaction> for MeasurementRepository {
    async fn add(
        &self,
        tx: &DatabaseTransaction,
        measurements: Vec<MeasurementEntity>,
    ) -> Result<(), GenerationError> {
        let measurements = measurements
            .into_iter()
            .map(|new| ActiveModel {
                unit: ActiveValue::Set(new.unit.into()),
                sub_system: ActiveValue::Set(new.sub_system),
                label: ActiveValue::Set(new.label),
                value: ActiveValue::Set(new.value),
                measured_at: ActiveValue::Set(new.monitored_at.into()),
                ..Default::default()
            })
            .collect::<Vec<ActiveModel>>();

        let _ = Measurements::insert_many(measurements)
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;

        Ok(())
    }

    async fn get(
        &self,
        tx: &DatabaseTransaction,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        sub_system: String,
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
