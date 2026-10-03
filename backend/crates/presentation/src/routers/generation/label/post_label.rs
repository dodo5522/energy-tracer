use crate::{error_mapper::ErrorMapperTrait, errors::ErrorResponse, routers::RouterState};
use axum::{Json, extract::State};
use http::StatusCode;
use layer_domain::entity::LabelEntity;
use layer_infra::{repository::label::LabelRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::label::AddLabelUseCase;
use serde::Deserialize;
use utoipa::ToSchema;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[derive(Deserialize, ToSchema)]
pub struct LabelPostRequest {
    /// ラベル
    pub label: String,
    /// 備考
    pub remark: String,
}

impl From<LabelPostRequest> for LabelEntity {
    fn from(body: LabelPostRequest) -> Self {
        LabelEntity {
            label: body.label,
            remark: body.remark,
        }
    }
}

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
    let factory = UnitOfWorkFactory::new(state.db.clone());
    AddLabelUseCase::new(LabelRepository {}, factory)
        .add(body.into())
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    Ok(StatusCode::CREATED)
}
