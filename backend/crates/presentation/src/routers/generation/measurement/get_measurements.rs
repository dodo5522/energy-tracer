use crate::{
    error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState,
    utilities::empty_string_as_none,
};
use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{DateTime, TimeDelta, Utc};
use http::StatusCode;
use layer_domain::entity::MeasurementEntity;
use layer_infra::{
    repository::measurement::MeasurementRepository, unit_of_work::UnitOfWorkFactory,
};
use layer_use_case::measurement::FetchMeasurementsUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

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
    /// 発電サブシステムの種類(e.g. 太陽光, 風力, ...)
    pub system: String,
    /// 発電状況のラベル(e.g. バッテリ電圧, パネル出力電流, 風車回転数, ...)
    pub label: String,
    /// 物理量
    pub value: f32,
    /// 物理量の単位(e.g. V, A, Wh, ...)
    pub unit: String,
    /// 計測日時
    pub at: DateTime<Utc>,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
#[schema(value_type = Vec<MeasurementItem>)]
#[serde(transparent)]
pub struct GetResponse(
    /// 物理量の値と計測日時
    pub Vec<MeasurementItem>,
);

impl From<Vec<MeasurementEntity>> for GetResponse {
    fn from(entities: Vec<MeasurementEntity>) -> Self {
        Self(
            entities
                .into_iter()
                .map(|e| MeasurementItem {
                    system: e.system,
                    label: e.label,
                    value: e.value,
                    unit: e.unit.into(),
                    at: e.measured_at,
                })
                .collect(),
        )
    }
}

#[utoipa::path(
    get,
    tag = "Generation - Measurement",
    description = "Get measurements with the specified parameters",
    path = "/generation/measurements",
    params(MeasurementFilter),
    responses(
        (status = 200, description = "OK", body = GetResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn get_measurements(
    State(state): State<RouterState>,
    Query(filter): Query<MeasurementFilter>,
) -> Result<(StatusCode, Json<GetResponse>), (StatusCode, Json<ErrorResponse>)> {
    let labels = if let Some(label) = filter.label {
        Some(vec![label])
    } else {
        None
    };
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let measurements = FetchMeasurementsUseCase::new(MeasurementRepository {}, factory)
        .fetch(filter.from, filter.to, filter.system, labels)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok((StatusCode::OK, Json(measurements.into())))
}
