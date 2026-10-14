use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};

use crate::{
    core::domain::users::{
        entities::{ActiveModel, Column, Entity, Model},
        repositories::UsersRepository,
    },
    error::AppError,
};

pub struct UsersRepositoryImpl {
    db: DatabaseConnection,
}

impl UsersRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl UsersRepository for UsersRepositoryImpl {
    async fn create(&self, new_user: ActiveModel) -> Result<Model, AppError> {
        match new_user.insert(&self.db).await {
            Ok(model) => Ok(model),
            Err(_) => Err(AppError::Validation("Failed to create user.".to_string())),
        }
    }

    async fn count_by_role(&self, role: &str) -> Result<u64, AppError> {
        Entity::find()
            .filter(Column::Role.eq(role))
            .count(&self.db)
            .await
            .map_err(|error| AppError::Internal(anyhow::Error::new(error)))
    }
}
