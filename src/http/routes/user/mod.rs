use axum::Form;
use axum_valid::Garde;
use garde::Validate;
use serde::Deserialize;

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

pub async fn register(Garde(Form(data)): Garde<Form<RegisterUser>>) {
    todo!("perform registration")
}
