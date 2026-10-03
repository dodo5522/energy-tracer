use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use http::StatusCode;
use layer_domain::entity::LabelEntity;
use layer_infra::{repository::label::LabelRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::label::UpdateLabelUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[derive(serde::Deserialize, utoipa::IntoParams)]
pub struct UpdateLabelQuery {
    pub remark: String,
}

#[utoipa::path(
    put,
    tag = "Generation - Label",
    description = "Update the specified label",
    path = "/generation/labels/{label}",
    params(
        UpdateLabelQuery,
        ("label", description = "label name"),
    ),
    responses(
        (status = 204, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn put_label(
    State(state): State<RouterState>,
    Path(label): Path<String>,
    Query(query): Query<UpdateLabelQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    UpdateLabelUseCase::new(LabelRepository {}, factory)
        .update(LabelEntity {
            label,
            remark: query.remark,
        })
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::OK)
}
