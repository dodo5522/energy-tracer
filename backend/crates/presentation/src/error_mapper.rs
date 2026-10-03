use super::errors::ErrorResponse;
use axum::Json;
use axum::http::StatusCode;
use layer_use_case::interface::GenerationError;

pub trait ErrorMapperTrait {
    fn map_to_bad_request<E: std::fmt::Display>(e: E) -> (StatusCode, Json<ErrorResponse>) {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: format!("{e}"),
            }),
        )
    }

    fn map_to_internal_server_error<E: std::fmt::Display>(
        e: E,
    ) -> (StatusCode, Json<ErrorResponse>) {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: format!("{e}"),
            }),
        )
    }

    fn map_generation_error(e: GenerationError) -> (StatusCode, Json<ErrorResponse>) {
        let (status_code, msg) = match e {
            GenerationError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            GenerationError::NotImplemented(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            GenerationError::Unknown(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            GenerationError::InvalidUnit(msg) => (StatusCode::BAD_REQUEST, msg),
            GenerationError::DbError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (
            status_code,
            Json(ErrorResponse {
                message: msg.to_string(),
            }),
        )
    }
}
