use crate::value_object::Unit;
use chrono::{DateTime, Utc};

/// 発電計測情報エンティティ
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementEntity {
    /// 物理量の値
    pub value: f32,
    /// 物理量の単位(e.g. V, A, Wh, ...)
    pub unit: Unit,
    /// 発電サブシステムの種類(e.g. 太陽光, 風力, ...)
    pub system: String,
    /// 発電状況のラベル(e.g. バッテリ電圧, パネル出力電流, 風車回転数, ...)
    pub label: String,
    /// 発電状況の計測日時
    pub monitored_at: DateTime<Utc>,
}
