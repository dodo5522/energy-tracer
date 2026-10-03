use crate::error_mapper::ErrorMapperTrait;
use crate::errors::ErrorResponse;
use crate::routers::RouterState;
use axum::Json;
use axum::extract::{Path, State};
use http::StatusCode;
use layer_domain::entity::LabelEntity;
use layer_infra::repository::label::LabelRepository;
use layer_infra::unit_of_work::UnitOfWorkFactory;
use layer_use_case::{interface::GenerationError, label::FetchLabelsUseCase};
use serde::Serialize;
use utoipa::ToSchema;

struct ErrorMapper {}
impl ErrorMapperTrait for ErrorMapper {}

#[derive(Serialize, ToSchema)]
pub struct LabelItem {
    /// ラベル
    pub label: String,
    /// 備考
    pub remark: String,
}

impl From<&LabelEntity> for LabelItem {
    fn from(e: &LabelEntity) -> Self {
        let owned = e.to_owned();
        Self {
            label: owned.label,
            remark: owned.remark,
        }
    }
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
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let found = FetchLabelsUseCase::new(LabelRepository {}, factory)
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
