use std::sync::Arc;

use chrono::Utc;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    prelude::DateTimeWithTimeZone,
};

use crate::{
    core::{
        application::posts::dto::CreatePostInput,
        domain::posts::{
            entities::{ActiveModel, Model},
            repositories::PostsRepository,
        },
    },
    error::AppError,
};

pub struct CreatePost {
    repository: Arc<dyn PostsRepository + Send + Sync>,
}

impl CreatePost {
    pub fn new(repository: Arc<dyn PostsRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    fn today(&self) -> DateTimeWithTimeZone {
        Utc::now().fixed_offset()
    }

    pub async fn execute(&self, model_post: CreatePostInput) -> Result<Model, AppError> {
        let model = ActiveModel {
            id: NotSet,
            thread_id: Set(model_post.thread_id),
            local_number: Set(model_post.local_number),
            is_op: Set(model_post.is_op),
            body: Set(model_post.body),
            image_id: Set(model_post.image_id),
            poster_id: Set(model_post.poster_id),
            ip_hash: Set(model_post.ip_hash),
            is_sage: Set(model_post.is_sage),
            is_deleted: Set(false),
            deleted_at: Set(None),
            created_at: Set(self.today()),
        };

        self.repository.create(model).await
    }
}
