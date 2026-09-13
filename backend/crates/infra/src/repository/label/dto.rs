use crate::models::labels::{ActiveModel, Model};
use layer_domain::entity::LabelEntity;
use sea_orm::ActiveValue;

impl From<LabelEntity> for ActiveModel {
    fn from(e: LabelEntity) -> Self {
        Self {
            label: ActiveValue::Set(e.label),
            remark: ActiveValue::Set(e.remark),
            ..Default::default()
        }
    }
}

impl From<&LabelEntity> for ActiveModel {
    fn from(e: &LabelEntity) -> Self {
        Self {
            label: ActiveValue::Set(e.label.to_owned()),
            remark: ActiveValue::Set(e.remark.to_owned()),
            ..Default::default()
        }
    }
}

impl From<Model> for LabelEntity {
    fn from(m: Model) -> Self {
        Self {
            label: m.label,
            remark: m.remark,
        }
    }
}
