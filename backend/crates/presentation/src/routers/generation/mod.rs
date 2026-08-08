use crate::routers::RouterState;
use axum::{
    Router,
    routing::{delete, get, post},
};

pub(crate) mod label;
pub(crate) mod measurement;
pub(crate) mod sub_system;
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
            "/sub_systems",
            post(sub_system::post_sub_system).get(sub_system::get_sub_systems),
        ))
        .merge(
            Router::new().route(
                "/sub_systems/{system}",
                delete(sub_system::delete_sub_system)
                    .get(sub_system::get_sub_system)
                    .put(sub_system::update_sub_system),
            ),
        )
        .merge(Router::new().route(
            "/sub_systems/{system}/measurements",
            get(sub_system::get_measurements_under_system),
        ))
        .merge(Router::new().route(
            "/sub_systems/{system}/labels/{label}/measurements",
            get(sub_system::get_measurements_under_system_and_label),
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
