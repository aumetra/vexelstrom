use super::routes;
use axum::routing;

pub fn construct() -> axum::Router {
    axum::Router::new().nest(
        "/api/v1",
        axum::Router::new().route("/register", routing::post(routes::user::register)),
    )
}
