use super::entities::{ActiveModel, Model};
use crate::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn create(&self, new_user: ActiveModel) -> Result<Model, AppError>;
    async fn update(&self, update: ActiveModel) -> Result<Model, AppError>;
    async fn count_by_role(&self, role: &str) -> Result<u64, AppError>;
    async fn auth(&self, username: String, password: String) -> Result<Model, AppError>;
    async fn find_by_username(&self, username: String) -> Result<Option<Model>, AppError>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Model>, AppError>;
}
