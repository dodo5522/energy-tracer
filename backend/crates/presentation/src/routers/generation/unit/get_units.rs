use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{Json, extract::State};
use http::StatusCode;
use layer_domain::entity::UnitEntity;
use layer_infra::{repository::unit::UnitRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::unit::FetchUnitsUseCase;
use serde::Serialize;
use utoipa::ToSchema;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[derive(Serialize, ToSchema)]
pub struct UnitItem {
    /// 物理量の単位
    pub unit: String,
    /// 備考
    pub remark: String,
}

impl From<UnitEntity> for UnitItem {
    fn from(u: UnitEntity) -> Self {
        Self {
            unit: u.unit.into(),
            remark: u.remark,
        }
    }
}

#[utoipa::path(
    get,
    tag = "Generation - Unit",
    description = "Get existing units",
    path = "/generation/units",
    responses(
        (status = 200, description = "OK", body = Vec<UnitItem>),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_units(
    State(state): State<RouterState>,
) -> Result<(StatusCode, Json<Vec<UnitItem>>), (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let units = FetchUnitsUseCase::new(UnitRepository {}, factory)
        .fetch(None::<&String>)
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    Ok((
        StatusCode::OK,
        Json(units.into_iter().map(UnitItem::from).collect()),
    ))
}
