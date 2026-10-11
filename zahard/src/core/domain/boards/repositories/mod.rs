use super::entities::{ActiveModel, Model};
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait BoardsRepository: Send + Sync {
    async fn create(&self, new_board: ActiveModel) -> Result<Model, AppError>;
}
