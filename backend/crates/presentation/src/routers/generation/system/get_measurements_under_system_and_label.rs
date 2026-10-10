use crate::error_mapper::ErrorMapperTrait;
use crate::{errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::{DateTime, Utc};
use http::StatusCode;
use layer_domain::entity::MeasurementEntity;
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

#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
pub struct MeasurementPathFilter {
    /// サブシステム
    #[param(example = "コントローラ")]
    pub system: String,
    /// ラベル
    #[param(example = "バッテリ電圧")]
    pub label: String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct MeasurementItem {
    /// 物理量
    pub value: f32,
    /// 物理量の単位(e.g. V, A, Wh, ...)
    pub unit: String,
    /// 計測日時
    pub at: DateTime<Utc>,
}

impl From<MeasurementEntity> for MeasurementItem {
    fn from(entity: MeasurementEntity) -> Self {
        Self {
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
    pub values: Vec<MeasurementItem>,
}

impl From<Vec<MeasurementEntity>> for Response {
    fn from(measurements: Vec<MeasurementEntity>) -> Self {
        Self {
            values: measurements
                .into_iter()
                .map(MeasurementItem::from)
                .collect(),
        }
    }
}

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get measurements under the sub system and label with range of date time",
    path = "/generation/systems/{system}/labels/{label}/measurements",
    params(MeasurementRangeFilter, MeasurementPathFilter),
    responses(
        // (status = 200, description = "OK", body = GetResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn get_measurements_under_system_and_label(
    State(state): State<RouterState>,
    Query(query): Query<MeasurementRangeFilter>,
    Path(path): Path<MeasurementPathFilter>,
) -> Result<(StatusCode, Json<Response>), (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let measurements = FetchMeasurementsUseCase::new(MeasurementRepository {}, factory)
        .fetch(
            query.from,
            query.to,
            Some(path.system),
            Some(vec![path.label]),
        )
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    Ok((StatusCode::OK, Json(measurements.into())))
}
