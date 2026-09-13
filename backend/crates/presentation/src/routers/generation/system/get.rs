use chrono::{DateTime, TimeDelta, Utc};
use layer_domain::entity::SystemEntity;

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
pub struct SystemItem {
    /// 発電サブシステムの種類
    pub system: String,
    /// 備考
    pub remark: String,
}

impl From<SystemEntity> for SystemItem {
    fn from(e: SystemEntity) -> Self {
        Self {
            system: e.system,
            remark: e.remark,
        }
    }
}

impl From<&SystemEntity> for SystemItem {
    fn from(e: &SystemEntity) -> Self {
        let owned = e.to_owned();
        Self {
            system: owned.system,
            remark: owned.remark,
        }
    }
}

impl From<SystemItem> for SystemEntity {
    fn from(system_item: SystemItem) -> Self {
        Self {
            system: system_item.system,
            remark: system_item.remark,
        }
    }
}
