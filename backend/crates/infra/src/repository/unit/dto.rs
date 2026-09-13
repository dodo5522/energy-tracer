use crate::models::units::{ActiveModel, Model};
use layer_domain::entity::UnitEntity;
use layer_use_case::interface::GenerationError;
use sea_orm::ActiveValue;

impl From<&UnitEntity> for ActiveModel {
    fn from(e: &UnitEntity) -> Self {
        Self {
            unit: ActiveValue::set(e.unit.to_owned().into()),
            remark: ActiveValue::set(e.remark.to_owned()),
            ..Default::default()
        }
    }
}

impl From<UnitEntity> for ActiveModel {
    fn from(e: UnitEntity) -> Self {
        Self {
            unit: ActiveValue::set(e.unit.into()),
            remark: ActiveValue::set(e.remark),
            ..Default::default()
        }
    }
}

impl TryFrom<Model> for UnitEntity {
    type Error = GenerationError;

    fn try_from(m: Model) -> Result<Self, Self::Error> {
        let unit = m.unit;
        Ok(Self {
            unit: unit
                .to_owned()
                .try_into()
                .map_err(|_| GenerationError::InvalidUnit(unit))?,
            remark: m.remark.into(),
        })
    }
}
