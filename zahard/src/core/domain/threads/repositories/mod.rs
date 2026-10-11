use super::entities::{ActiveModel, Model};
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait ThreadsRepository: Send + Sync {
    async fn create(&self, new_thread: ActiveModel) -> Result<Model, AppError>;
}
