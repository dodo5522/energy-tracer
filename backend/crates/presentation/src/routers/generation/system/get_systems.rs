use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{Json, extract::State};
use http::StatusCode;
use layer_domain::entity::SystemEntity;
use layer_infra::{repository::system::SubSystemRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::system::FetchSystemUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct SystemItem {
    /// 発電サブシステムの種類
    pub system: String,
    /// 備考
    pub remark: String,
}

#[derive(serde::Serialize, utoipa::ToSchema)]
#[schema(value_type = Vec<SystemItem>)]
#[serde(transparent)]
pub struct GetSystemsResponse(pub Vec<SystemItem>);

impl From<Vec<SystemEntity>> for GetSystemsResponse {
    fn from(systems: Vec<SystemEntity>) -> Self {
        GetSystemsResponse(
            systems
                .into_iter()
                .map(|s| SystemItem {
                    system: s.system,
                    remark: s.remark,
                })
                .collect(),
        )
    }
}

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get existing sub systems",
    path = "/generation/systems",
    responses(
        (status = 200, description = "OK", body = GetSystemsResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_systems(
    State(state): State<RouterState>,
) -> Result<(StatusCode, Json<GetSystemsResponse>), (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let systems = FetchSystemUseCase::new(SubSystemRepository {}, factory)
        .fetch(None::<&String>)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok((StatusCode::OK, Json(systems.into())))
}
