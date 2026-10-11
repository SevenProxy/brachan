use super::entities::{ActiveModel, Model};
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait PostReferencesRepository: Send + Sync {
    async fn create(&self, new_post_reference: ActiveModel) -> Result<Model, AppError>;
}
