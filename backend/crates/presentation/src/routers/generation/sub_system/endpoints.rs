use super::{
    get::{SubSystemItem, SubSystemMeasurementLabelFilter, SubSystemMeasurementRangeFilter},
    post::SubSystemPostRequest,
    put::UpdateSubSystemQuery,
};
use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use layer_domain::entity::SubSystemEntity;
use layer_infra::repository::measurement::MeasurementRepository;
use layer_infra::{repository::sub_system::SubSystemRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::interface::GenerationError;
use layer_use_case::measurement::RecordMeasurementUseCase;
use layer_use_case::sub_system::SubSystemUseCase;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[utoipa::path(
    post,
    tag = "Generation - Sub System",
    description = "Create a new sub system",
    path = "/generation/sub_systems",
    request_body = SubSystemPostRequest,
    responses(
        (status = 201, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn post_sub_system(
    State(state): State<RouterState>,
    Json(body): Json<SubSystemPostRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let system = SubSystemEntity {
        system: body.sub_system,
        remark: body.remark,
    };
    println!("Inserting sub system record: {:?}", system);

    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = SubSystemUseCase::new(repo, factory);

    if let Err(e) = use_case.create(system).await {
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
    tag = "Generation - Sub System",
    description = "Update the specified sub system",
    path = "/generation/sub_systems/{system}",
    params(
        UpdateSubSystemQuery,
        ("system", description = "Sub system name"),
    ),
    responses(
        (status = 204, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn update_sub_system(
    State(state): State<RouterState>,
    Path(system): Path<String>,
    Query(query): Query<UpdateSubSystemQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let system = SubSystemEntity {
        system: system,
        remark: query.remark,
    };
    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = SubSystemUseCase::new(repo, factory);
    let _ = use_case
        .update(system)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get existing sub systems",
    path = "/generation/sub_systems",
    responses(
        (status = 200, description = "OK", body = Vec<SubSystemItem>),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_sub_systems(
    State(state): State<RouterState>,
) -> Result<(StatusCode, Json<Vec<SubSystemItem>>), (StatusCode, Json<ErrorResponse>)> {
    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = SubSystemUseCase::new(repo, factory);
    let systems = use_case
        .get_all()
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    Ok((
        StatusCode::OK,
        Json(systems.into_iter().map(SubSystemItem::from).collect()),
    ))
}

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get specified sub system",
    path = "/generation/sub_systems/{system}",
    responses(
        (status = 200, description = "OK", body = SubSystemItem),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_sub_system(
    State(state): State<RouterState>,
    Path(system): Path<String>,
) -> Result<(StatusCode, Json<SubSystemItem>), (StatusCode, Json<ErrorResponse>)> {
    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = SubSystemUseCase::new(repo, factory);
    let found_system = use_case
        .get(&system)
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    if let Some(system) = found_system {
        Ok((StatusCode::OK, Json(system.into())))
    } else {
        Err(ErrorMapper::map_generation_error(
            GenerationError::NotFound(format!("Sub system '{system}' not found")),
        ))
    }
}

#[utoipa::path(
    delete,
    tag = "Generation - Sub System",
    description = "Delete specified sub system",
    path = "/generation/sub_systems/{system}",
    params(
        ("system", description = "Sub system name"),
    ),
    responses(
        (status = 204, description = "OK"),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn delete_sub_system(
    State(state): State<RouterState>,
    Path(system): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = SubSystemUseCase::new(repo, factory);

    let _ = use_case
        .delete(system)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get measurements under the sub system with range of date time",
    path = "/generation/sub_systems/{system}/measurements",
    params(SubSystemMeasurementRangeFilter),
    responses(
        // (status = 200, description = "OK", body = GetResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn get_measurements_under_system(
    State(state): State<RouterState>,
    Query(filter): Query<SubSystemMeasurementRangeFilter>,
) -> Result<
    (
        StatusCode,
        Json<crate::routers::generation::measurement::get::Response>,
    ),
    (StatusCode, Json<ErrorResponse>),
> {
    let repo = MeasurementRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = RecordMeasurementUseCase::new(repo, factory);
    // let measurement = use_case
    //     .get(id)
    //     .await
    //     .map_err(ErrorMapper::map_generation_error)?;

    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            message: "Measurement record not found".to_string(),
        }),
    ))
}

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get measurements under the sub system and label with range of date time",
    path = "/generation/sub_systems/{system}/labels/{label}/measurements",
    params(SubSystemMeasurementRangeFilter, SubSystemMeasurementLabelFilter),
    responses(
        // (status = 200, description = "OK", body = GetResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse)
    )
)]
pub async fn get_measurements_under_system_and_label(
    State(state): State<RouterState>,
    Query(query): Query<SubSystemMeasurementRangeFilter>,
    Path(path): Path<SubSystemMeasurementLabelFilter>,
) -> Result<
    (
        StatusCode,
        Json<crate::routers::generation::measurement::get::Response>,
    ),
    (StatusCode, Json<ErrorResponse>),
> {
    let repo = MeasurementRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = RecordMeasurementUseCase::new(repo, factory);
    // let measurement = use_case
    //     .get(id)
    //     .await
    //     .map_err(ErrorMapper::map_generation_error)?;

    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            message: "Measurement record not found".to_string(),
        }),
    ))
}
