use crate::routers::RouterState;
use axum::{
    Router,
    routing::{delete, get, post},
};

pub(crate) mod label;
pub(crate) mod measurement;
pub(crate) mod system;
pub(crate) mod unit;

pub fn route() -> Router<RouterState> {
    Router::<RouterState>::new()
        .merge(Router::new().route(
            "/measurements",
            post(measurement::post_measurements).get(measurement::get_measurements),
        ))
        .merge(Router::new().route("/labels", post(label::post_label).get(label::get_labels)))
        .merge(
            Router::new().route(
                "/labels/{label}",
                delete(label::delete_label)
                    .get(label::get_label)
                    .put(label::update_label),
            ),
        )
        .merge(Router::new().route(
            "/systems",
            post(system::post_system).get(system::get_systems),
        ))
        .merge(
            Router::new().route(
                "/systems/{system}",
                delete(system::delete_system)
                    .get(system::get_system)
                    .put(system::update_system),
            ),
        )
        .merge(Router::new().route(
            "/systems/{system}/measurements",
            get(system::get_measurements_under_system),
        ))
        .merge(Router::new().route(
            "/systems/{system}/labels/{label}/measurements",
            get(system::get_measurements_under_system_and_label),
        ))
        .merge(Router::new().route("/units", post(unit::post_unit).get(unit::get_units)))
        .merge(
            Router::new().route(
                "/units/{unit}",
                delete(unit::delete_unit)
                    .get(unit::get_unit)
                    .put(unit::update_unit),
            ),
        )
}
