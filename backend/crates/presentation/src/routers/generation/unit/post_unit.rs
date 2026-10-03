use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{Json, extract::State};
use http::StatusCode;
use layer_domain::entity::UnitEntity;
use layer_infra::{repository::unit::UnitRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::unit::AddUnitUseCase;
use serde::Deserialize;
use utoipa::ToSchema;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[derive(Deserialize, ToSchema)]
pub struct UnitPostRequest {
    /// 物理量の単位
    pub unit: String,
    /// 備考
    pub remark: String,
}

impl TryFrom<UnitPostRequest> for UnitEntity {
    type Error = (StatusCode, Json<ErrorResponse>);

    fn try_from(body: UnitPostRequest) -> Result<Self, Self::Error> {
        Ok(UnitEntity {
            unit: body
                .unit
                .try_into()
                .map_err(ErrorMapper::map_to_bad_request)?,
            remark: body.remark,
        })
    }
}

#[utoipa::path(
    post,
    tag = "Generation - Unit",
    description = "Create a new unit",
    path = "/generation/units",
    request_body = UnitPostRequest,
    responses(
        (status = 201, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn post_unit(
    State(state): State<RouterState>,
    Json(body): Json<UnitPostRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let factory = UnitOfWorkFactory::new(state.db.clone());
    AddUnitUseCase::new(UnitRepository {}, factory)
        .add(body.try_into()?)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::CREATED)
}
