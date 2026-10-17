use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use crate::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|error| AppError::Internal(anyhow::Error::new(error)))
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|error| AppError::Internal(anyhow::Error::new(error)))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
