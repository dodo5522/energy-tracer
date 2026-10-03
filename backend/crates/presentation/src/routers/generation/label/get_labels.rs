use crate::error_mapper::ErrorMapperTrait;
use crate::errors::ErrorResponse;
use crate::routers::RouterState;
use axum::Json;
use axum::extract::State;
use http::StatusCode;
use layer_domain::entity::LabelEntity;
use layer_infra::repository::label::LabelRepository;
use layer_infra::unit_of_work::UnitOfWorkFactory;
use layer_use_case::label::FetchLabelsUseCase;
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

impl From<LabelEntity> for LabelItem {
    fn from(e: LabelEntity) -> Self {
        Self {
            label: e.label,
            remark: e.remark,
        }
    }
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
    let factory = UnitOfWorkFactory::new(state.db.clone());
    let labels = FetchLabelsUseCase::new(LabelRepository {}, factory)
        .fetch(None::<&String>)
        .await
        .map_err(ErrorMapper::map_generation_error)?;
    let items = labels.into_iter().map(|e| LabelItem::from(e)).collect();
    Ok((StatusCode::OK, Json(items)))
}
