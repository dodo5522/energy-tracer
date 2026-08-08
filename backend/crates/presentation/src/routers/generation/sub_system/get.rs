use chrono::{DateTime, TimeDelta, Utc};
use layer_domain::entity::SubSystemEntity;

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
pub struct SubSystemMeasurementRangeFilter {
    /// 計測開始日時 (省略時は現在時刻の1時間前)
    #[param(example = "2026-06-26T11:34:56Z", required = false)]
    from: DateTime<Utc>,
    /// 計測終了日時 (省略時は現在時刻)
    #[param(example = "2026-06-26T21:34:56Z", required = false)]
    to: DateTime<Utc>,
}

impl Default for SubSystemMeasurementRangeFilter {
    fn default() -> Self {
        let to = Utc::now();
        let from = to - TimeDelta::hours(1);
        Self { to, from }
    }
}

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
pub struct SubSystemMeasurementLabelFilter {
    /// サブシステム
    #[param(example = "コントローラ")]
    pub system: String,
    /// ラベル
    #[param(example = "バッテリ電圧")]
    pub label: String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct SubSystemItem {
    /// 発電サブシステムの種類
    pub system: String,
    /// 備考
    pub remark: String,
}

impl From<SubSystemEntity> for SubSystemItem {
    fn from(e: SubSystemEntity) -> Self {
        Self {
            system: e.system,
            remark: e.remark,
        }
    }
}

impl From<SubSystemItem> for SubSystemEntity {
    fn from(sub_system_item: SubSystemItem) -> Self {
        Self {
            system: sub_system_item.system,
            remark: sub_system_item.remark,
        }
    }
}
