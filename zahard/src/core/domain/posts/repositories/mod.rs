use super::entities::{ActiveModel, Model};
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait PostsRepository: Send + Sync {
    async fn create(&self, new_post: ActiveModel) -> Result<Model, AppError>;
}
