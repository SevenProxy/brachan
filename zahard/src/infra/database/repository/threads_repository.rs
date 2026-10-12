use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{
    core::domain::threads::{
        entities::{ActiveModel, Model},
        repositories::ThreadsRepository,
    },
    error::AppError,
};

pub struct ThreadsRepositoryImpl {
    db: DatabaseConnection,
}

impl ThreadsRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl ThreadsRepository for ThreadsRepositoryImpl {
    async fn create(&self, new_thread: ActiveModel) -> Result<Model, AppError> {
        match new_thread.insert(&self.db).await {
            Ok(model) => Ok(model),
            Err(_) => Err(AppError::Validation("Failed to create thread.".to_string())),
        }
    }
}
