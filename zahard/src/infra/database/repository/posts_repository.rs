use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{
    core::domain::posts::{
        entities::{ActiveModel, Model},
        repositories::PostsRepository,
    },
    error::AppError,
};

pub struct PostsRepositoryImpl {
    db: DatabaseConnection,
}

impl PostsRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl PostsRepository for PostsRepositoryImpl {
    async fn create(&self, new_post: ActiveModel) -> Result<Model, AppError> {
        match new_post.insert(&self.db).await {
            Ok(model) => Ok(model),
            Err(_) => Err(AppError::Validation("Failed to create post.".to_string())),
        }
    }
}
