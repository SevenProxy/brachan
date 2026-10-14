use argon2::{Argon2, password_hash::PasswordHasher};
use chrono::Utc;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    prelude::DateTimeWithTimeZone,
};
use std::sync::Arc;

use crate::{
    core::{
        application::users::dto::CreateUserInput,
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

    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        Argon2::default()
            .hash_password(password.as_bytes())
            .map(|hash| hash.to_string())
            .map_err(|error| AppError::Internal(anyhow::Error::new(error)))
    }

    pub async fn execute(&self, model_user: CreateUserInput) -> Result<Model, AppError> {
        let password = self.hash_password(&model_user.password)?;
        let role = if self.repository.count().await? == 0 {
            "admin"
        } else {
            "user"
        };

        let model = ActiveModel {
            id: NotSet,
            username: Set(model_user.username),
            password: Set(password),
            role: Set(role.to_string()),
            is_active: Set(true),
            created_at: Set(self.today()),
        };

        self.repository.create(model).await
    }
}
