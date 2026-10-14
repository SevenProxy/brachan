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
