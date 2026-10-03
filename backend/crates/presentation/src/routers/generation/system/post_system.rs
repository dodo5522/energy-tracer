use crate::{errors::ErrorResponse, routers::RouterState};
use axum::{Json, extract::State, http::StatusCode};
use layer_domain::entity::SystemEntity;
use layer_infra::{repository::system::SubSystemRepository, unit_of_work::UnitOfWorkFactory};
use layer_use_case::system::AddSystemUseCase;

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct PostSystemRequest {
    /// 発電サブシステムの種類
    pub system: String,
    /// 備考
    pub remark: String,
}

#[utoipa::path(
    post,
    tag = "Generation - Sub System",
    description = "Create a new sub system",
    path = "/generation/systems",
    request_body = PostSystemRequest,
    responses(
        (status = 201, description = "OK"),
        (status = 400, description = "Bad request", body = ErrorResponse),
        (status = 500, description = "Internal Error", body = ErrorResponse),
    )
)]
pub async fn post_system(
    State(state): State<RouterState>,
    Json(body): Json<PostSystemRequest>,
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
