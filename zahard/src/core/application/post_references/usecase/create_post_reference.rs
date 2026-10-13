use std::sync::Arc;

use chrono::Utc;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    prelude::DateTimeWithTimeZone,
};

use crate::{
    core::{
        application::post_references::dto::CreatePostReferenceInput,
        domain::post_references::{
            entities::{ActiveModel, Model},
            repositories::PostReferencesRepository,
        },
    },
    error::AppError,
};

pub struct CreatePostReference {
    repository: Arc<dyn PostReferencesRepository + Send + Sync>,
}

impl CreatePostReference {
    pub fn new(repository: Arc<dyn PostReferencesRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    fn today(&self) -> DateTimeWithTimeZone {
        Utc::now().fixed_offset()
    }

    pub async fn execute(
        &self,
        model_post_reference: CreatePostReferenceInput,
    ) -> Result<Model, AppError> {
        let model = ActiveModel {
            post_id: Set(model_post_reference.post_id),
            referenced_id: Set(model_post_reference.referenced_id),
            thread_id: Set(model_post_reference.thread_id),
            created_at: Set(self.today()),
        };

        self.repository.create(model).await
    }
}