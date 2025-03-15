use super::{routes, state::AppState};
use axum::routing;

pub fn construct(state: AppState) -> axum::Router {
    axum::Router::new()
        .nest(
            "/api/v1",
            axum::Router::new().route("/register", routing::post(routes::user::register)),
        )
        .with_state(state)
}
