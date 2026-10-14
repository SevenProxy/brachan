use argon2::{Argon2, password_hash::PasswordHasher};

use crate::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|error| AppError::Internal(anyhow::Error::new(error)))
}
