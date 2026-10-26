use chrono::Utc;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    prelude::DateTimeWithTimeZone,
};
use std::sync::Arc;

use crate::{
    core::{
        application::users::password::hash_password,
        domain::users::{
            entities::{ActiveModel, Model},
            repositories::UsersRepository,
        },
    },
    error::AppError,
};

pub struct BootstrapAdmin {
    repository: Arc<dyn UsersRepository + Send + Sync>,
}

impl BootstrapAdmin {
    pub fn new(repository: Arc<dyn UsersRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    fn today(&self) -> DateTimeWithTimeZone {
        Utc::now().fixed_offset()
    }

    pub async fn execute(&self) -> Result<Option<Model>, AppError> {
        if self.repository.count_by_role("admin").await? > 0 {
            return Ok(None);
        }

        let username = std::env::var("ADMIN_USERNAME")
            .map_err(|_| AppError::Validation("ADMIN_USERNAME must be set".to_string()))?;
        let password = std::env::var("ADMIN_PASSWORD")
            .map_err(|_| AppError::Validation("ADMIN_PASSWORD must be set".to_string()))?;
        let password = hash_password(&password)?;

        let model = ActiveModel {
            id: NotSet,
            username: Set(username),
            password: Set(password),
            role: Set("admin".to_string()),
            is_active: Set(true),
            created_at: Set(self.today()),
        };

        self.repository.create(model).await.map(Some)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::BootstrapAdmin;
    use crate::core::application::users::{
        password::verify_password,
        usecase::test_support::{MockUsersRepository, make_user},
    };

    #[tokio::test]
    async fn skips_bootstrap_when_an_admin_exists() {
        let repository = Arc::new(MockUsersRepository::with_users(vec![make_user(
            1, "admin", "hash", "admin", true,
        )]));

        let created = BootstrapAdmin::new(repository).execute().await.unwrap();

        assert!(created.is_none());
    }

    #[tokio::test]
    async fn creates_an_admin_from_the_environment() {
        unsafe {
            std::env::set_var("ADMIN_USERNAME", "root");
            std::env::set_var("ADMIN_PASSWORD", "secret");
        }
        let repository = Arc::new(MockUsersRepository::new());

        let admin = BootstrapAdmin::new(repository.clone())
            .execute()
            .await
            .unwrap()
            .unwrap();

        assert_eq!(admin.username, "root");
        assert_eq!(admin.role, "admin");
        assert!(admin.is_active);
        assert!(verify_password("secret", &admin.password).unwrap());
        assert_eq!(repository.users().len(), 1);
    }
}
