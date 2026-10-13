use std::sync::Arc;

use chrono::Utc;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    prelude::DateTimeWithTimeZone,
};

use crate::{
    core::{
        application::threads::dto::CreateThreadInput,
        domain::threads::{
            entities::{ActiveModel, Model},
            repositories::ThreadsRepository,
        },
    },
    error::AppError,
};

pub struct CreateThread {
    repository: Arc<dyn ThreadsRepository + Send + Sync>,
}

impl CreateThread {
    pub fn new(repository: Arc<dyn ThreadsRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    fn today(&self) -> DateTimeWithTimeZone {
        Utc::now().fixed_offset()
    }

    pub async fn execute(&self, model_thread: CreateThreadInput) -> Result<Model, AppError> {
        let model = ActiveModel {
            id: NotSet,
            board_id: Set(model_thread.board_id),
            subject: Set(model_thread.subject),
            is_pinned: Set(false),
            is_locked: Set(false),
            is_archived: Set(false),
            reply_count: Set(0),
            image_count: Set(0),
            bumped_at: Set(self.today()),
            created_at: Set(self.today()),
        };

        self.repository.create(model).await
    }
}
