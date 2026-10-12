use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{
    core::domain::post_references::{
        entities::{ActiveModel, Model},
        repositories::PostReferencesRepository,
    },
    error::AppError,
};

pub struct PostReferencesRepositoryImpl {
    db: DatabaseConnection,
}

impl PostReferencesRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl PostReferencesRepository for PostReferencesRepositoryImpl {
    async fn create(&self, new_post_reference: ActiveModel) -> Result<Model, AppError> {
        match new_post_reference.insert(&self.db).await {
            Ok(model) => Ok(model),
            Err(_) => Err(AppError::Validation(
                "Failed to create post reference.".to_string(),
            )),
        }
    }
}
