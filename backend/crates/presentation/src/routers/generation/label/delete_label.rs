use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use layer_infra::{repository::label::LabelRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::label::DeleteLabelUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[utoipa::path(
    delete,
    tag = "Generation - Label",
    description = "Delete specified label",
    path = "/generation/labels/{label}",
    params(
        ("label", description = "label name"),
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn delete_label(
    State(state): State<RouterState>,
    Path(label): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    DeleteLabelUseCase::new(LabelRepository {}, factory)
        .delete(label)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::NO_CONTENT)
}
