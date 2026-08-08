use crate::models::sub_systems::{ActiveModel, Model};
use layer_domain::entity::SubSystemEntity;
use sea_orm::ActiveValue;

impl From<SubSystemEntity> for ActiveModel {
    fn from(e: SubSystemEntity) -> Self {
        Self {
            sub_system: ActiveValue::Set(e.system),
            remark: ActiveValue::Set(e.remark),
            ..Default::default()
        }
    }
}

impl From<&SubSystemEntity> for ActiveModel {
    fn from(e: &SubSystemEntity) -> Self {
        Self {
            sub_system: ActiveValue::Set(e.to_owned().system),
            remark: ActiveValue::Set(e.to_owned().remark),
            ..Default::default()
        }
    }
}

impl From<&Model> for SubSystemEntity {
    fn from(m: &Model) -> Self {
        let model = m.to_owned();
        Self {
            system: model.sub_system,
            remark: model.remark,
        }
    }
}

impl From<Model> for SubSystemEntity {
    fn from(m: Model) -> Self {
        Self {
            system: m.sub_system,
            remark: m.remark,
        }
    }
}
