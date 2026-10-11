use super::entities::{ActiveModel, Model};
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait ImagesRepository: Send + Sync {
    async fn create(&self, new_image: ActiveModel) -> Result<Model, AppError>;
}
