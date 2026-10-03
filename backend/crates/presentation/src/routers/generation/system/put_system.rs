use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use http::StatusCode;
use layer_domain::entity::SystemEntity;
use layer_infra::{repository::system::SubSystemRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::system::UpdateSystemUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[derive(serde::Deserialize, utoipa::IntoParams)]
pub struct PutSystemRequestQuery {
    pub remark: String,
}

#[utoipa::path(
    put,
    tag = "Generation - Sub System",
    description = "Update the specified sub system",
    path = "/generation/systems/{system}",
    params(
        PutSystemRequestQuery,
        ("system", description = "Sub system name"),
    ),
    responses(
        (status = 204, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn put_system(
    State(state): State<RouterState>,
    Path(system): Path<String>,
    Query(query): Query<PutSystemRequestQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let system = SystemEntity {
        system,
        remark: query.remark,
    };
    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = UpdateSystemUseCase::new(repo, factory);
    let _ = use_case
        .update(&system)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::NO_CONTENT)
}
