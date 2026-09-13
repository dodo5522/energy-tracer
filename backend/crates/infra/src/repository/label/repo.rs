use crate::{
    error_mapper::ErrorMapperTrait,
    models::{labels::*, prelude::*},
};
use layer_domain::entity::LabelEntity;
use layer_use_case::interface::{GenerationError, LabelRepositoryTrait};
use sea_orm::{DatabaseTransaction, QueryFilter, entity::prelude::*};

pub struct LabelRepository {}

impl ErrorMapperTrait for LabelRepository {}

#[async_trait::async_trait]
impl LabelRepositoryTrait<DatabaseTransaction> for LabelRepository {
    async fn add(
        &self,
        tx: &DatabaseTransaction,
        label: LabelEntity,
    ) -> Result<i64, GenerationError> {
        let result = Labels::insert::<ActiveModel>(label.into())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        Ok(result.last_insert_id)
    }

    async fn find(
        &self,
        tx: &DatabaseTransaction,
        label: Option<&String>,
    ) -> Result<Vec<LabelEntity>, GenerationError> {
        if let Some(label) = label {
            let found = Labels::find()
                .filter(Column::Label.eq(label))
                .one(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            if let Some(label) = found {
                Ok(vec![label.into()])
            } else {
                Err(GenerationError::NotFound(label.into()))
            }
        } else {
            let labels = Labels::find()
                .all(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            labels
                .into_iter()
                .map(|label| Ok(label.into()))
                .collect::<Result<Vec<LabelEntity>, _>>()
        }
    }

    async fn update(
        &self,
        tx: &DatabaseTransaction,
        label: &LabelEntity,
    ) -> Result<i64, GenerationError> {
        let result = Labels::update::<ActiveModel>(label.into())
            .exec(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;
        Ok(result.id)
    }

    async fn delete(&self, tx: &DatabaseTransaction, label: String) -> Result<(), GenerationError> {
        let found = Labels::find()
            .filter(Column::Label.eq(&label))
            .one(tx)
            .await
            .map_err(Self::map_db_to_generation_error)?;

        if let Some(l) = found {
            let result = Labels::delete::<ActiveModel>(l.into())
                .exec(tx)
                .await
                .map_err(Self::map_db_to_generation_error)?;
            if result.rows_affected > 0 {
                Ok(())
            } else {
                Err(GenerationError::Unknown(label))
            }
        } else {
            Err(GenerationError::NotFound(label))
        }
    }
}
