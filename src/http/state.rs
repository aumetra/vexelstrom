use crate::db::PgPool;
use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: PgPool,

    // required for `axum_valid`
    pub unit: (),
}
