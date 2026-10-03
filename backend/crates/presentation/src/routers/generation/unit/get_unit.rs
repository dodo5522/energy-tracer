use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, State},
};
use http::StatusCode;
use layer_domain::entity::UnitEntity;
use layer_infra::{repository::unit::UnitRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::{interface::GenerationError, unit::FetchUnitsUseCase};
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

impl From<&UnitEntity> for UnitItem {
    fn from(u: &UnitEntity) -> Self {
        let u = u.to_owned();
        Self {
            unit: u.unit.into(),
            remark: u.remark,
        }
    }
}

#[utoipa::path(
    get,
    tag = "Generation - Unit",
    description = "Get existing unit",
    path = "/generation/units/{unit}",
    params(
        ("unit", description = "unit name"),
    ),
    responses(
        (status = 200, description = "OK", body = UnitItem),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_unit(
    State(state): State<RouterState>,
    Path(unit): Path<String>,
) -> Result<(StatusCode, Json<UnitItem>), (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let found = FetchUnitsUseCase::new(UnitRepository {}, factory)
        .fetch(Some(&unit))
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    if let Some(unit) = found.first() {
        Ok((StatusCode::OK, Json(unit.into())))
    } else {
        Err(ErrorMapper::map_generation_error(
            GenerationError::NotFound(format!("Unit '{unit}' not found")),
        ))
    }
}
