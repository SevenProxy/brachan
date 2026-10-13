use std::sync::Arc;

use chrono::Utc;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    prelude::DateTimeWithTimeZone,
};

use crate::{
    core::{
        application::boards::dto::CreateBoardInput,
        domain::boards::{
            entities::{ActiveModel, Model},
            repositories::BoardsRepository,
        },
    },
    error::AppError,
};

pub struct CreateBoard {
    repository: Arc<dyn BoardsRepository + Send + Sync>,
}

impl CreateBoard {
    pub fn new(repository: Arc<dyn BoardsRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    fn today(&self) -> DateTimeWithTimeZone {
        Utc::now().fixed_offset()
    }

    pub async fn execute(&self, model_board: CreateBoardInput) -> Result<Model, AppError> {
        let model = ActiveModel {
            id: NotSet,
            slug: Set(model_board.slug),
            name: Set(model_board.name),
            description: Set(model_board.description),
            bump_limit: Set(300),
            max_threads: Set(200),
            is_nsfw: Set(false),
            created_at: Set(self.today()),
        };

        self.repository.create(model).await
    }
}
