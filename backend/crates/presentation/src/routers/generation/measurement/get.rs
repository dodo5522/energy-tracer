use crate::utilities::empty_string_as_none;
use chrono::{DateTime, TimeDelta, Utc};

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
#[serde(default)]
pub struct MeasurementFilter {
    /// 計測開始日時 (省略時は現在時刻の1時間前)
    #[param(example = "2026-06-26T11:34:56Z", required = false)]
    pub from: DateTime<Utc>,
    /// 計測終了日時 (省略時は現在時刻)
    #[param(example = "2026-06-26T21:34:56Z", required = false)]
    pub to: DateTime<Utc>,
    /// サブシステム
    #[param(example = "コントローラ", required = false)]
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub system: Option<String>,
    /// ラベル
    #[param(example = "バッテリ電圧", required = false)]
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub label: Option<String>,
}

impl Default for MeasurementFilter {
    fn default() -> Self {
        let to = Utc::now();
        let from = to - TimeDelta::hours(-1);
        Self {
            to,
            from,
            system: None,
            label: None,
        }
    }
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct MeasurementItem {
    /// 物理量
    pub value: f32,
    /// 物理量の単位(e.g. V, A, Wh, ...)
    pub unit: String,
    /// 計測日時
    pub monitored_at: DateTime<Utc>,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct Response {
    /// 発電サブシステムの種類(e.g. 太陽光, 風力, ...)
    pub system: String,
    /// 発電状況のラベル(e.g. バッテリ電圧, パネル出力電流, 風車回転数, ...)
    pub label: String,
    /// 物理量の値と計測日時
    pub values: Vec<MeasurementItem>,
}
