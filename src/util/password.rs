use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString},
};

pub async fn hash(password: String) -> eyre::Result<String> {
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());
        let password_hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    })
    .await?
}

pub async fn verify(password: String, password_hash: String) -> eyre::Result<bool> {
    tokio::task::spawn_blocking(move || {
        let hash = PasswordHash::new(&password_hash)?;
        let is_valid = Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok();

        Ok(is_valid)
    })
    .await?
}
