use crate::routers::{RouterState, generation, health};
use axum::{Router, extract::State};
use http::{HeaderValue, Method};
use sea_orm::DatabaseConnection;
use std::io::{Error, ErrorKind};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

fn cors(allowed_origins: Vec<String>) -> Result<CorsLayer, Error> {
    let origins = allowed_origins
        .into_iter()
        .map(|origin| {
            origin
                .parse::<HeaderValue>()
                .map_err(|e| Error::new(ErrorKind::Other, e))
        })
        .collect::<Result<Vec<HeaderValue>, _>>()?;

    Ok(CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any))
}

pub fn route(allowed_origins: Vec<String>, db: DatabaseConnection) -> Result<Router, Error> {
    Ok(Router::<RouterState>::new()
        .merge(SwaggerUi::new("/docs/swagger").url("/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/docs/redoc", ApiDoc::openapi()))
        .nest("/health", health::route())
        .nest("/generation", generation::route())
        .layer(cors(allowed_origins)?)
        .with_state(RouterState { db }))
}

#[derive(OpenApi)]
#[openapi(paths(
    health::checker::check_health,
    generation::measurement::get_measurements,
    generation::measurement::post_measurements,
    generation::label::delete_label,
    generation::label::post_label,
    generation::label::get_label,
    generation::label::get_labels,
    generation::label::update_label,
    generation::sub_system::delete_sub_system,
    generation::sub_system::post_sub_system,
    generation::sub_system::get_sub_system,
    generation::sub_system::get_sub_systems,
    generation::sub_system::get_measurements_under_system,
    generation::sub_system::get_measurements_under_system_and_label,
    generation::sub_system::update_sub_system,
    generation::unit::delete_unit,
    generation::unit::post_unit,
    generation::unit::get_unit,
    generation::unit::get_units,
    generation::unit::update_unit,
))]
pub(crate) struct ApiDoc {}
