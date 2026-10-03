use super::MeasurementRangeFilter;
use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, Query, State},
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
pub struct MeasurementPathFilterSystem {
    #[param(example = "コントローラ", required = true)]
    system: String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct MeasurementItemWithLabel {
    /// 発電状況のラベル(e.g. バッテリ電圧, パネル出力電流, 風車回転数, ...)
    pub label: String,
    /// 物理量
    pub value: f32,
    /// 物理量の単位(e.g. V, A, Wh, ...)
    pub unit: String,
    /// 計測日時
    pub at: DateTime<Utc>,
}

impl From<MeasurementEntity> for MeasurementItemWithLabel {
    fn from(entity: MeasurementEntity) -> Self {
        Self {
            label: entity.label,
            value: entity.value,
            unit: entity.unit.into(),
            at: entity.measured_at,
        }
    }
}

#[derive(serde::Serialize, utoipa::ToSchema)]
#[serde(transparent)]
pub struct Response {
    /// 物理量の値と計測日時
    pub values: Vec<MeasurementItemWithLabel>,
}

impl From<Vec<MeasurementEntity>> for Response {
    fn from(entities: Vec<MeasurementEntity>) -> Self {
        Self {
            values: entities
                .into_iter()
                .map(MeasurementItemWithLabel::from)
                .collect(),
        }
    }
}

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get measurements under the sub system with range of date time",
    path = "/generation/systems/{system}/measurements",
    params(MeasurementRangeFilter, MeasurementPathFilterSystem),
    responses(
        (status = 200, description = "OK", body = Response),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn get_measurements_under_system(
    State(state): State<RouterState>,
    Query(filter): Query<MeasurementRangeFilter>,
    Path(path): Path<MeasurementPathFilterSystem>,
) -> Result<(StatusCode, Json<Response>), (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let measurements = FetchMeasurementsUseCase::new(MeasurementRepository {}, factory)
        .fetch(filter.from, filter.to, Some(path.system), None)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok((StatusCode::OK, Json(measurements.into())))
}
