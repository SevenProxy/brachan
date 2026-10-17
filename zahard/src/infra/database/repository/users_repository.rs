use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};

use crate::{
    core::{
        application::users::password::verify_password,
        domain::users::{
            entities::{ActiveModel, Column, Entity, Model},
            repositories::UsersRepository,
        },
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

    async fn auth(&self, username: String, password: String) -> Result<Model, AppError> {
        let user = Entity::find()
            .filter(Column::Username.eq(username.as_str()))
            .one(&self.db)
            .await
            .map_err(|error| AppError::Internal(anyhow::Error::new(error)))?
            .ok_or(AppError::Unauthorized)?;

        if verify_password(&password, &user.password)? {
            Ok(user)
        } else {
            Err(AppError::Unauthorized)
        }
    }
}
