use super::{get::LabelItem, post::LabelPostRequest, put::UpdateLabelQuery};
use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use layer_domain::entity::LabelEntity;
use layer_infra::{repository::label::LabelRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::{
    interface::GenerationError,
    label::{AddLabelUseCase, DeleteLabelUseCase, FetchLabelsUseCase, UpdateLabelUseCase},
};

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[utoipa::path(
    post,
    tag = "Generation - Label",
    description = "Create a new label",
    path = "/generation/labels",
    request_body = LabelPostRequest,
    responses(
        (status = 201, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn post_label(
    State(state): State<RouterState>,
    Json(body): Json<LabelPostRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let label = LabelEntity {
        label: body.label,
        remark: body.remark,
    };
    println!("Inserting label record: {:?}", label);

    let repo = LabelRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = AddLabelUseCase::new(repo, factory);

    if let Err(e) = use_case.add(label).await {
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
pub async fn update_label(
    State(state): State<RouterState>,
    Path(label): Path<String>,
    Query(query): Query<UpdateLabelQuery>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let repo = LabelRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = UpdateLabelUseCase::new(repo, factory);
    let _ = use_case
        .update(LabelEntity {
            label,
            remark: query.remark,
        })
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::OK)
}

#[utoipa::path(
    get,
    tag = "Generation - Label",
    description = "Get existing labels",
    path = "/generation/labels",
    responses(
        (status = 200, description = "OK", body = Vec<LabelItem>),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_labels(
    State(state): State<RouterState>,
) -> Result<(StatusCode, Json<Vec<LabelItem>>), (StatusCode, Json<ErrorResponse>)> {
    let repo = LabelRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = FetchLabelsUseCase::new(repo, factory);
    let labels = use_case
        .fetch(None::<&String>)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    let items = labels
        .into_iter()
        .map(|e| LabelItem::try_from(e).map_err(ErrorMapper::map_to_internal_server_error))
        .collect::<Result<Vec<LabelItem>, _>>()?;
    Ok((StatusCode::OK, Json(items)))
}

#[utoipa::path(
    get,
    tag = "Generation - Label",
    description = "Get specified label",
    path = "/generation/labels/{label}",
    params(
        ("label", description = "label name"),
    ),
    responses(
        (status = 200, description = "OK", body = LabelItem),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn get_label(
    State(state): State<RouterState>,
    Path(label): Path<String>,
) -> Result<(StatusCode, Json<LabelItem>), (StatusCode, Json<ErrorResponse>)> {
    let repo = LabelRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = FetchLabelsUseCase::new(repo, factory);
    let found = use_case
        .fetch(Some(&label))
        .await
        .map_err(ErrorMapper::map_generation_error)?;

    if let Some(label) = found.first() {
        Ok((StatusCode::OK, Json(label.into())))
    } else {
        Err(ErrorMapper::map_generation_error(
            GenerationError::NotFound(format!("Label '{label}' not found")),
        ))
    }
}

#[utoipa::path(
    delete,
    tag = "Generation - Label",
    description = "Delete specified label",
    path = "/generation/labels/{label}",
    params(
        ("label", description = "label name"),
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn delete_label(
    State(state): State<RouterState>,
    Path(label): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let repo = LabelRepository {};
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let use_case = DeleteLabelUseCase::new(repo, factory);
    let _ = use_case
        .delete(label)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::NO_CONTENT)
}
