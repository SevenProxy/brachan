use std::sync::Arc;

use chrono::Utc;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    prelude::DateTimeWithTimeZone,
};

use crate::{
    core::{
        application::images::dto::CreateImageInput,
        domain::images::{
            entities::{ActiveModel, Model},
            repositories::ImagesRepository,
        },
    },
    error::AppError,
};

pub struct CreateImage {
    repository: Arc<dyn ImagesRepository + Send + Sync>,
}

impl CreateImage {
    pub fn new(repository: Arc<dyn ImagesRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    fn today(&self) -> DateTimeWithTimeZone {
        Utc::now().fixed_offset()
    }

    pub async fn execute(&self, model_image: CreateImageInput) -> Result<Model, AppError> {
        let model = ActiveModel {
            id: NotSet,
            disk_name: Set(model_image.disk_name),
            thumb_name: Set(model_image.thumb_name),
            mime_type: Set(model_image.mime_type),
            size_bytes: Set(model_image.size_bytes),
            width: Set(model_image.width),
            height: Set(model_image.height),
            sha256: Set(model_image.sha256),
            created_at: Set(self.today()),
        };

        self.repository.create(model).await
    }
}
