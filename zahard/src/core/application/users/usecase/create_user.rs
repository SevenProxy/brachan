use chrono::Utc;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    prelude::DateTimeWithTimeZone,
};
use std::sync::Arc;

use crate::{
    core::{
        application::users::{dto::UserInput, password::hash_password},
        domain::users::{
            entities::{ActiveModel, Model},
            repositories::UsersRepository,
        },
    },
    error::AppError,
};

pub struct CreateUser {
    repository: Arc<dyn UsersRepository + Send + Sync>,
}

impl CreateUser {
    pub fn new(repository: Arc<dyn UsersRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    fn today(&self) -> DateTimeWithTimeZone {
        Utc::now().fixed_offset()
    }

    pub async fn execute(&self, model_user: UserInput) -> Result<Model, AppError> {
        let existing = self
            .repository
            .find_by_username(model_user.username.clone())
            .await?;

        if existing.is_some() {
            return Err(AppError::Conflict("Username already taken".to_string()));
        }

        let password = hash_password(&model_user.password)?;

        let model = ActiveModel {
            id: NotSet,
            username: Set(model_user.username),
            password: Set(password),
            role: Set("user".to_string()),
            is_active: Set(true),
            created_at: Set(self.today()),
        };

        self.repository.create(model).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::CreateUser;
    use crate::{
        core::application::users::{
            dto::UserInput,
            password::verify_password,
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
    async fn creates_a_user_with_a_hashed_password() {
        let repository = Arc::new(MockUsersRepository::new());

        let user = CreateUser::new(repository.clone())
            .execute(input("alice", "secret"))
            .await
            .unwrap();

        assert_eq!(user.username, "alice");
        assert_eq!(user.role, "user");
        assert!(user.is_active);
        assert_ne!(user.password, "secret");
        assert!(verify_password("secret", &user.password).unwrap());
        assert_eq!(repository.users().len(), 1);
    }

    #[tokio::test]
    async fn rejects_an_existing_username() {
        let repository = Arc::new(MockUsersRepository::with_users(vec![make_user(
            1, "alice", "hash", "user", true,
        )]));

        let error = CreateUser::new(repository)
            .execute(input("alice", "secret"))
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Conflict(_)));
    }
}
