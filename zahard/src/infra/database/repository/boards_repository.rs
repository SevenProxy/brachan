use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::{
    core::domain::boards::{
        entities::{ActiveModel, Model},
        repositories::BoardsRepository,
    },
    error::AppError,
};

pub struct BoardsRepositoryImpl {
    db: DatabaseConnection,
}

impl BoardsRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl BoardsRepository for BoardsRepositoryImpl {
    async fn create(&self, new_board: ActiveModel) -> Result<Model, AppError> {
        match new_board.insert(&self.db).await {
            Ok(model) => Ok(model),
            Err(_) => Err(AppError::Validation("Failed to create board.".to_string())),
        }
    }
}
