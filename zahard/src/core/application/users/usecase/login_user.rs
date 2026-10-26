use std::sync::Arc;

use crate::{
    core::{
        application::users::{dto::UserInput, token::generate_token},
        domain::users::repositories::UsersRepository,
    },
    error::AppError,
};

pub struct LoginUser {
    repository: Arc<dyn UsersRepository + Send + Sync>,
}

impl LoginUser {
    pub fn new(repository: Arc<dyn UsersRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_login: UserInput) -> Result<String, AppError> {
        let user = self
            .repository
            .auth(user_login.username, user_login.password)
            .await?;

        generate_token(&user)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use jsonwebtoken::{DecodingKey, Validation, decode};

    use super::LoginUser;
    use crate::{
        core::application::users::{
            dto::UserInput,
            password::hash_password,
            token::Claims,
            usecase::test_support::{MockUsersRepository, make_user},
        },
        error::AppError,
    };

    fn input(username: &str, password: &str) -> UserInput {
        UserInput {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    #[tokio::test]
    async fn returns_a_token_for_valid_credentials() {
        let password_hash = hash_password("secret").unwrap();
        let repository = Arc::new(MockUsersRepository::with_users(vec![make_user(
            1,
            "alice",
            &password_hash,
            "user",
            true,
        )]));

        unsafe { std::env::set_var("JWT_SECRET", "test-secret") };

        let token = LoginUser::new(repository)
            .execute(input("alice", "secret"))
            .await
            .unwrap();

        let data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(b"test-secret"),
            &Validation::default(),
        )
        .unwrap();

        assert_eq!(data.claims.sub, 1);
        assert_eq!(data.claims.username, "alice");
        assert_eq!(data.claims.role, "user");
    }

    #[tokio::test]
    async fn rejects_invalid_credentials() {
        let password_hash = hash_password("secret").unwrap();
        let repository = Arc::new(MockUsersRepository::with_users(vec![make_user(
            1,
            "alice",
            &password_hash,
            "user",
            true,
        )]));

        let error = LoginUser::new(repository)
            .execute(input("alice", "wrong"))
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Unauthorized));
    }

    #[tokio::test]
    async fn rejects_an_unknown_user() {
        let repository = Arc::new(MockUsersRepository::new());

        let error = LoginUser::new(repository)
            .execute(input("ghost", "secret"))
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Unauthorized));
    }
}
