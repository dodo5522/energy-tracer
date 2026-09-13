use super::{get::UnitItem, post::UnitPostRequest, put::UpdateUnitQuery};
use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use layer_domain::entity::UnitEntity;
use layer_infra::{repository::unit::UnitRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::{
    interface::GenerationError,
    unit::{AddUnitUseCase, DeleteUnitUseCase, FetchUnitsUseCase, UpdateUnitUseCase},
};

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

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
    let unit = UnitEntity {
        unit: body
            .unit
            .try_into()
            .map_err(ErrorMapper::map_to_bad_request)?,
        remark: body.remark,
    };
    println!("Inserting unit record: {:?}", unit);

    let repo = UnitRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = AddUnitUseCase::new(repo, factory);

    if let Err(e) = use_case.add(unit).await {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: format!("{e}"),
            }),
        ))
    } else {
        Ok(StatusCode::CREATED)
    }
}

#[utoipa::path(
    put,
    tag = "Generation - Unit",
    description = "Update the specified unit",
    path = "/generation/units/{unit}",
    params(
        UpdateUnitQuery,
        ("unit", description = "unit name"),
    ),
    responses(
        (status = 204, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn update_unit(
    State(state): State<RouterState>,
    Path(unit): Path<String>,
    Query(query): Query<UpdateUnitQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let repo = UnitRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = UpdateUnitUseCase::new(repo, factory);

    let _ = use_case
        .update(UnitEntity {
            unit: unit.try_into().map_err(ErrorMapper::map_to_bad_request)?,
            remark: query.remark,
        })
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::NO_CONTENT)
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
    let repo = UnitRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = FetchUnitsUseCase::new(repo, factory);
    let units = use_case
        .fetch(None::<&String>)
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    Ok((
        StatusCode::OK,
        Json(units.into_iter().map(UnitItem::from).collect()),
    ))
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
    let repo = UnitRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = FetchUnitsUseCase::new(repo, factory);
    let found = use_case
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
    let repo = UnitRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = DeleteUnitUseCase::new(repo, factory);
    let _ = use_case
        .delete(unit)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::NO_CONTENT)
}
