use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, State},
};
use http::StatusCode;
use layer_domain::entity::SystemEntity;
use layer_infra::{repository::system::SubSystemRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::{interface::GenerationError, system::FetchSystemUseCase};

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct GetSystemResponse {
    /// 発電サブシステムの種類
    pub system: String,
    /// 備考
    pub remark: String,
}

impl From<&SystemEntity> for GetSystemResponse {
    fn from(entity: &SystemEntity) -> Self {
        let e = entity.to_owned();
        GetSystemResponse {
            system: e.system,
            remark: e.remark,
        }
    }
}

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get specified sub system",
    path = "/generation/systems/{system}",
    responses(
        (status = 200, description = "OK", body = GetSystemResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_system(
    State(state): State<RouterState>,
    Path(system): Path<String>,
) -> Result<(StatusCode, Json<GetSystemResponse>), (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let found = FetchSystemUseCase::new(SubSystemRepository {}, factory)
        .fetch(Some(&system))
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    if let Some(system) = found.first() {
        Ok((StatusCode::OK, Json(system.into())))
    } else {
        Err(ErrorMapper::map_generation_error(
            GenerationError::NotFound(format!("Sub system '{system}' not found")),
        ))
    }
}
