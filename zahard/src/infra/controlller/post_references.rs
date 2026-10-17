use axum::{Json, extract::State, http::StatusCode};

use crate::{
    core::{
        application::post_references::{
            dto::CreatePostReferenceInput, usecase::create_post_reference::CreatePostReference,
        },
        domain::post_references::entities::Model,
    },
    error::AppError,
    state::AppState,
};

pub struct PostReferenceController;

impl PostReferenceController {
    pub async fn create(
        State(state): State<AppState>,
        Json(input): Json<CreatePostReferenceInput>,
    ) -> Result<(StatusCode, Json<Model>), AppError> {
        let post_reference = CreatePostReference::new(state.post_references)
            .execute(input)
            .await?;

        Ok((StatusCode::CREATED, Json(post_reference)))
    }
}
