use chrono::{DateTime, Utc};
use layer_domain::{entity::MeasurementEntity, value_object::UnitError};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct MeasurementValue {
    /// 物理量の値
    pub value: f32,
    /// 物理量の単位(e.g. V, A, Wh, ...)
    pub unit: String,
    /// 発電システムの種類(e.g. 太陽光, 風力, ...)
    pub system: String,
    /// 発電状況のラベル(e.g. バッテリ電圧, パネル出力電流, 風車回転数, ...)
    pub label: String,
}

#[derive(Deserialize, ToSchema)]
pub struct PostMeasurementRequest {
    /// 発電状況の計測値
    pub values: Vec<MeasurementValue>,
    /// 発電状況の計測日時
    pub monitored_at: DateTime<Utc>,
}

impl TryFrom<PostMeasurementRequest> for Vec<MeasurementEntity> {
    type Error = UnitError;

    fn try_from(input: PostMeasurementRequest) -> Result<Self, Self::Error> {
        input
            .values
            .into_iter()
            .map(|item| {
                Ok(MeasurementEntity {
                    value: item.value,
                    unit: item.unit.try_into()?,
                    system: item.system,
                    label: item.label,
                    monitored_at: input.monitored_at,
                })
            })
            .collect::<Result<Vec<MeasurementEntity>, Self::Error>>()
    }
}
