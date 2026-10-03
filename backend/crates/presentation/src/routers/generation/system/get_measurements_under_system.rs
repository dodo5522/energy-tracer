use crate::error_mapper::ErrorMapperTrait;
use crate::errors::ErrorResponse;
use crate::routers::RouterState;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::{DateTime, TimeDelta, Utc};
use http::StatusCode;
use layer_infra::{
    repository::measurement::MeasurementRepository, unit_of_work::UnitOfWorkFactory,
};
use layer_use_case::measurement::FetchMeasurementsUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
pub struct MeasurementRangeFilter {
    /// 計測開始日時 (省略時は現在時刻の1時間前)
    #[param(example = "2026-06-26T11:34:56Z", required = false)]
    from: DateTime<Utc>,
    /// 計測終了日時 (省略時は現在時刻)
    #[param(example = "2026-06-26T21:34:56Z", required = false)]
    to: DateTime<Utc>,
}

impl Default for MeasurementRangeFilter {
    fn default() -> Self {
        let to = Utc::now();
        let from = to - TimeDelta::hours(1);
        Self { to, from }
    }
}

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
pub struct MeasurementPathFilter {
    #[param(example = "コントローラ", required = true)]
    system: String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct MeasurementItem {
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
#[serde(transparent)]
pub struct Response {
    /// 物理量の値と計測日時
    pub values: Vec<MeasurementItem>,
}

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get measurements under the sub system with range of date time",
    path = "/generation/systems/{system}/measurements",
    params(MeasurementRangeFilter, MeasurementPathFilter),
    responses(
        (status = 200, description = "OK", body = Response),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn get_measurements_under_system(
    State(state): State<RouterState>,
    Query(filter): Query<MeasurementRangeFilter>,
    Path(path): Path<MeasurementPathFilter>,
) -> Result<(StatusCode, Json<Response>), (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let measurement = FetchMeasurementsUseCase::new(MeasurementRepository {}, factory)
        .fetch(filter.from, filter.to, Some(path.system), None)
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    // TODO

    Err((
        StatusCode::OK,
        Json(ErrorResponse {
            message: String::new(),
        }),
    ))
}
