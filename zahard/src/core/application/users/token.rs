use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

use crate::{core::domain::users::entities::Model, error::AppError};

const DEFAULT_EXPIRATION_HOURS: i64 = 24;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub role: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn generate_token(user: &Model) -> Result<String, AppError> {
    let secret = jwt_secret()?;
    generate_token_with(user, &secret, expiration_hours())
}

fn generate_token_with(
    user: &Model,
    secret: &str,
    expiration_hours: i64,
) -> Result<String, AppError> {
    let now = Utc::now();
    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        role: user.role.clone(),
        iat: now.timestamp(),
        exp: (now + Duration::hours(expiration_hours)).timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|error| AppError::Internal(anyhow::Error::new(error)))
}

fn jwt_secret() -> Result<String, AppError> {
    std::env::var("JWT_SECRET")
        .map_err(|_| AppError::Validation("JWT_SECRET must be set".to_string()))
}

fn expiration_hours() -> i64 {
    std::env::var("JWT_EXPIRATION_HOURS")
        .ok()
        .and_then(|value| value.parse::<i64>().ok())
        .filter(|hours| *hours > 0)
        .unwrap_or(DEFAULT_EXPIRATION_HOURS)
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{DecodingKey, Validation, decode};

    use super::*;

    fn user() -> Model {
        Model {
            id: 1,
            username: "admin".to_string(),
            password: "hash".to_string(),
            role: "admin".to_string(),
            is_active: true,
            created_at: Utc::now().fixed_offset(),
        }
    }

    #[test]
    fn generates_a_token_with_expected_claims() {
        let token = generate_token_with(&user(), "test-secret", 1).unwrap();

        let data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(b"test-secret"),
            &Validation::default(),
        )
        .unwrap();

        assert_eq!(data.claims.sub, 1);
        assert_eq!(data.claims.username, "admin");
        assert_eq!(data.claims.role, "admin");
        assert!(data.claims.exp > data.claims.iat);
    }

    #[test]
    fn rejects_a_token_signed_with_another_secret() {
        let token = generate_token_with(&user(), "test-secret", 1).unwrap();

        let result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(b"another-secret"),
            &Validation::default(),
        );

        assert!(result.is_err());
    }
}
