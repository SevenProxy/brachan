use sea_orm::ActiveValue::Set;
use std::sync::Arc;

use crate::{
    core::{
        application::users::{dto::UpdateUserApplication, password::hash_password},
        domain::users::{
            entities::{ActiveModel, Model},
            repositories::UsersRepository,
        },
    },
    error::AppError,
};

pub struct UpdateUser {
    repository: Arc<dyn UsersRepository + Send + Sync>,
}

impl UpdateUser {
    pub fn new(repository: Arc<dyn UsersRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, id: i64, input: UpdateUserApplication) -> Result<Model, AppError> {
        let user = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        let mut active: ActiveModel = user.into();

        if let Some(username) = input.username {
            if username.trim().is_empty() {
                return Err(AppError::Validation("Username cannot be empty".to_string()));
            }
            active.username = Set(username);
        }
        if let Some(password) = input.password {
            if password.is_empty() {
                return Err(AppError::Validation("Password cannot be empty".to_string()));
            }
            let hashed = hash_password(&password)?;
            active.password = Set(hashed);
        }
        if let Some(role) = input.role {
            if role.trim().is_empty() {
                return Err(AppError::Validation("Role cannot be empty".to_string()));
            }
            active.role = Set(role);
        }
        if let Some(is_active) = input.is_active {
            active.is_active = Set(is_active);
        }

        self.repository.update(active).await
    }

    pub async fn execute_by_model(&self, user: ActiveModel) -> Result<Model, AppError> {
        self.repository.update(user).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::UpdateUser;
    use crate::{
        core::{
            application::users::{
                dto::UpdateUserApplication,
                password::{hash_password, verify_password},
                usecase::test_support::{MockUsersRepository, make_user},
            },
            domain::users::entities::ActiveModel,
        },
        error::AppError,
    };

    fn empty_update() -> UpdateUserApplication {
        UpdateUserApplication {
            username: None,
            password: None,
            role: None,
            is_active: None,
        }
    }

    #[tokio::test]
    async fn updates_the_username() {
        let repository = Arc::new(MockUsersRepository::with_users(vec![make_user(
            1, "alice", "hash", "user", true,
        )]));

        let user = UpdateUser::new(repository.clone())
            .execute(
                1,
                UpdateUserApplication {
                    username: Some("bob".to_string()),
                    ..empty_update()
                },
            )
            .await
            .unwrap();

        assert_eq!(user.username, "bob");
        assert_eq!(repository.users()[0].username, "bob");
    }

    #[tokio::test]
    async fn updates_the_password_with_a_hash() {
        let repository = Arc::new(MockUsersRepository::with_users(vec![make_user(
            1, "alice", "hash", "user", true,
        )]));

        let user = UpdateUser::new(repository)
            .execute(
                1,
                UpdateUserApplication {
                    password: Some("new-secret".to_string()),
                    ..empty_update()
                },
            )
            .await
            .unwrap();

        assert_ne!(user.password, "new-secret");
        assert!(verify_password("new-secret", &user.password).unwrap());
    }

    #[tokio::test]
    async fn updates_the_role_and_active_status() {
        let repository = Arc::new(MockUsersRepository::with_users(vec![make_user(
            1, "alice", "hash", "user", true,
        )]));

        let user = UpdateUser::new(repository)
            .execute(
                1,
                UpdateUserApplication {
                    role: Some("admin".to_string()),
                    is_active: Some(false),
                    ..empty_update()
                },
            )
            .await
            .unwrap();

        assert_eq!(user.role, "admin");
        assert!(!user.is_active);
    }

    #[tokio::test]
    async fn rejects_an_unknown_user() {
        let repository = Arc::new(MockUsersRepository::new());

        let error = UpdateUser::new(repository)
            .execute(99, empty_update())
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn rejects_empty_username_password_and_role() {
        let repository = Arc::new(MockUsersRepository::with_users(vec![make_user(
            1, "alice", "hash", "user", true,
        )]));
        let usecase = UpdateUser::new(repository);

        let cases = [
            UpdateUserApplication {
                username: Some("   ".to_string()),
                ..empty_update()
            },
            UpdateUserApplication {
                password: Some(String::new()),
                ..empty_update()
            },
            UpdateUserApplication {
                role: Some("   ".to_string()),
                ..empty_update()
            },
        ];

        for case in cases {
            let error = usecase.execute(1, case).await.unwrap_err();
            assert!(matches!(error, AppError::Validation(_)));
        }
    }

    #[tokio::test]
    async fn execute_by_model_updates_the_stored_user() {
        let password_hash = hash_password("secret").unwrap();
        let repository = Arc::new(MockUsersRepository::with_users(vec![make_user(
            1,
            "alice",
            &password_hash,
            "user",
            true,
        )]));
        let mut active: ActiveModel = repository.users()[0].clone().into();
        active.username = sea_orm::ActiveValue::Set("bob".to_string());

        let user = UpdateUser::new(repository)
            .execute_by_model(active)
            .await
            .unwrap();

        assert_eq!(user.username, "bob");
    }
}
