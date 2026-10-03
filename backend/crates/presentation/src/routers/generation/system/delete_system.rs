use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, State},
};
use http::StatusCode;
use layer_infra::{repository::system::SubSystemRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::system::DeleteSystemUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[utoipa::path(
    delete,
    tag = "Generation - Sub System",
    description = "Delete specified sub system",
    path = "/generation/systems/{system}",
    params(
        ("system", description = "Sub system name"),
    ),
    responses(
        (status = 204, description = "OK"),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn delete_system(
    State(state): State<RouterState>,
    Path(system): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    DeleteSystemUseCase::new(SubSystemRepository {}, factory)
        .delete(system)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::NO_CONTENT)
}
