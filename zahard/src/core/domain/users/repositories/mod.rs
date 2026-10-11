use super::entities::{ActiveModel, Model};
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn create(&self, new_user: ActiveModel) -> Result<Model, AppError>;
}
