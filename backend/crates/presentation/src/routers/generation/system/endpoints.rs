use super::{
    get::{SubSystemMeasurementLabelFilter, SubSystemMeasurementRangeFilter, SystemItem},
    post::SubSystemPostRequest,
    put::UpdateSubSystemQuery,
};
use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use layer_domain::entity::SystemEntity;
use layer_infra::repository::measurement::MeasurementRepository;
use layer_infra::{repository::system::SubSystemRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::{
    interface::GenerationError,
    measurement::RecordMeasurementUseCase,
    system::{AddSystemUseCase, DeleteSystemUseCase, FetchSystemUseCase, UpdateSystemUseCase},
};

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[utoipa::path(
    post,
    tag = "Generation - Sub System",
    description = "Create a new sub system",
    path = "/generation/systems",
    request_body = SubSystemPostRequest,
    responses(
        (status = 201, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn post_system(
    State(state): State<RouterState>,
    Json(body): Json<SubSystemPostRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let system = SystemEntity {
        system: body.system,
        remark: body.remark,
    };
    println!("Inserting sub system record: {:?}", system);

    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = AddSystemUseCase::new(repo, factory);

    if let Err(e) = use_case.add(system).await {
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
    path = "/generation/systems/{system}",
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
pub async fn update_system(
    State(state): State<RouterState>,
    Path(system): Path<String>,
    Query(query): Query<UpdateSubSystemQuery>,
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

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get existing sub systems",
    path = "/generation/systems",
    responses(
        (status = 200, description = "OK", body = Vec<SystemItem>),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_systems(
    State(state): State<RouterState>,
) -> Result<(StatusCode, Json<Vec<SystemItem>>), (StatusCode, Json<ErrorResponse>)> {
    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = FetchSystemUseCase::new(repo, factory);
    let systems = use_case
        .fetch(None::<&String>)
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    Ok((
        StatusCode::OK,
        Json(systems.into_iter().map(SystemItem::from).collect()),
    ))
}

#[utoipa::path(
    get,
    tag = "Generation - Sub System",
    description = "Get specified sub system",
    path = "/generation/systems/{system}",
    responses(
        (status = 200, description = "OK", body = SystemItem),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_system(
    State(state): State<RouterState>,
    Path(system): Path<String>,
) -> Result<(StatusCode, Json<SystemItem>), (StatusCode, Json<ErrorResponse>)> {
    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = FetchSystemUseCase::new(repo, factory);
    let found = use_case
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

#[utoipa::path(
    delete,
    tag = "Generation - Sub System",
    description = "Delete specified sub system",
    path = "/generation/systems/{system}",
    params(
        ("system", description = "Sub system name"),
    ),
    responses(
        (status = 204, description = "OK"),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn delete_system(
    State(state): State<RouterState>,
    Path(system): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let repo = SubSystemRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = DeleteSystemUseCase::new(repo, factory);
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
    path = "/generation/systems/{system}/measurements",
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
    path = "/generation/systems/{system}/labels/{label}/measurements",
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
