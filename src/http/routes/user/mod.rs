use crate::{
    db::{PgPool, changesets::NewUser, schema::users},
    http::error::AppError,
};
use axum::{Form, debug_handler, extract::State, http::StatusCode};
use axum_valid::Garde;
use diesel_async::RunQueryDsl;
use garde::Validate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Validate)]
pub struct RegisterUser {
    #[garde(length(min = 1))]
    username: String,
    #[garde(email)]
    email: String,
    #[garde(length(min = 1))]
    password: String,
    #[garde(matches(password))]
    password_confirm: String,
}

#[debug_handler(state = crate::http::state::AppState)]
pub async fn register(
    State(pool): State<PgPool>,
    Garde(Form(data)): Garde<Form<RegisterUser>>,
) -> Result<StatusCode, AppError> {
    let password = crate::util::password::hash(data.password).await?;
    let user = NewUser {
        id: Uuid::now_v7(),
        username: &data.username,
        email: &data.email,
        password: &password,
    };

    {
        let mut conn = pool.get().await?;
        diesel::insert_into(users::table)
            .values(user)
            .execute(&mut conn)
            .await?;
    }

    Ok(StatusCode::CREATED)
}
