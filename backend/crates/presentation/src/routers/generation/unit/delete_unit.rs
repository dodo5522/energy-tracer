use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use layer_infra::{repository::unit::UnitRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::unit::DeleteUnitUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[utoipa::path(
    delete,
    tag = "Generation - Unit",
    description = "Delete existing unit",
    path = "/generation/units/{unit}",
    params(
        ("unit", description = "unit name"),
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn delete_unit(
    State(state): State<RouterState>,
    Path(unit): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    DeleteUnitUseCase::new(UnitRepository {}, factory)
        .delete(unit)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::NO_CONTENT)
}
