use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use http::StatusCode;
use layer_domain::entity::UnitEntity;
use layer_infra::{repository::unit::UnitRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::unit::UpdateUnitUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[derive(serde::Deserialize, utoipa::IntoParams)]
pub struct UpdateUnitQuery {
    pub remark: String,
}

#[utoipa::path(
    put,
    tag = "Generation - Unit",
    description = "Update the specified unit",
    path = "/generation/units/{unit}",
    params(
        UpdateUnitQuery,
        ("unit", description = "unit name"),
    ),
    responses(
        (status = 204, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn put_unit(
    State(state): State<RouterState>,
    Path(unit): Path<String>,
    Query(query): Query<UpdateUnitQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    UpdateUnitUseCase::new(UnitRepository {}, factory)
        .update(UnitEntity {
            unit: unit.try_into().map_err(ErrorMapper::map_to_bad_request)?,
            remark: query.remark,
        })
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::NO_CONTENT)
}
