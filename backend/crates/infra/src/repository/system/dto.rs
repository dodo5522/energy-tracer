use crate::models::systems::{ActiveModel, Model};
use layer_domain::entity::SystemEntity;
use sea_orm::ActiveValue;

impl From<SystemEntity> for ActiveModel {
    fn from(e: SystemEntity) -> Self {
        Self {
            system: ActiveValue::Set(e.system),
            remark: ActiveValue::Set(e.remark),
            ..Default::default()
        }
    }
}

impl From<&SystemEntity> for ActiveModel {
    fn from(e: &SystemEntity) -> Self {
        Self {
            system: ActiveValue::Set(e.to_owned().system),
            remark: ActiveValue::Set(e.to_owned().remark),
            ..Default::default()
        }
    }
}

impl From<&Model> for SystemEntity {
    fn from(m: &Model) -> Self {
        let model = m.to_owned();
        Self {
            system: model.system,
            remark: model.remark,
        }
    }
}

impl From<Model> for SystemEntity {
    fn from(m: Model) -> Self {
        Self {
            system: m.system,
            remark: m.remark,
        }
    }
}
