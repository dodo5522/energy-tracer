use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use http::StatusCode;
use layer_domain::{entity::MeasurementEntity, value_object::UnitError};
use layer_infra::{
    repository::measurement::MeasurementRepository, unit_of_work::UnitOfWorkFactory,
};
use layer_use_case::measurement::RecordMeasurementUseCase;
use serde::Deserialize;
use utoipa::ToSchema;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

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
                    measured_at: input.monitored_at,
                })
            })
            .collect::<Result<Vec<MeasurementEntity>, Self::Error>>()
    }
}

#[utoipa::path(
    post,
    tag = "Generation - Measurement",
    description = "Create a new measurement record",
    path = "/generation/measurements",
    request_body = PostMeasurementRequest,
    responses(
        (status = 201, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn post_measurements(
    State(state): State<RouterState>,
    Json(body): Json<PostMeasurementRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let created = RecordMeasurementUseCase::new(MeasurementRepository {}, factory)
        .record(body.try_into().map_err(ErrorMapper::map_to_bad_request)?)
        .await;

    match created {
        Ok(()) => Ok(StatusCode::CREATED),
        Err(error) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: format!("{error}"),
            }),
        )),
    }
}
