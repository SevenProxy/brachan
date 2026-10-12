use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{
    core::domain::images::{
        entities::{ActiveModel, Model},
        repositories::ImagesRepository,
    },
    error::AppError,
};

pub struct ImagesRepositoryImpl {
    db: DatabaseConnection,
}

impl ImagesRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl ImagesRepository for ImagesRepositoryImpl {
    async fn create(&self, new_image: ActiveModel) -> Result<Model, AppError> {
        match new_image.insert(&self.db).await {
            Ok(model) => Ok(model),
            Err(_) => Err(AppError::Validation("Failed to create image.".to_string())),
        }
    }
}
