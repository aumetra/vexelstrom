use axum::{http::StatusCode, response::IntoResponse};

pub struct AppError(pub eyre::Report);

impl<E> From<E> for AppError
where
    E: Into<eyre::Report>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let Self(error) = self;
        error!(?error, "stuff failed");

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
