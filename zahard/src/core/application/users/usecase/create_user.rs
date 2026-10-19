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
