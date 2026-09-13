use layer_domain::entity::LabelEntity;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LabelItem {
    /// ラベル
    pub label: String,
    /// 備考
    pub remark: String,
}

impl From<LabelEntity> for LabelItem {
    fn from(e: LabelEntity) -> Self {
        Self {
            label: e.label,
            remark: e.remark,
        }
    }
}

impl From<&LabelEntity> for LabelItem {
    fn from(e: &LabelEntity) -> Self {
        let owned = e.to_owned();
        Self {
            label: owned.label,
            remark: owned.remark,
        }
    }
}

impl From<LabelItem> for LabelEntity {
    fn from(i: LabelItem) -> Self {
        Self {
            label: i.label,
            remark: i.remark,
        }
    }
}
