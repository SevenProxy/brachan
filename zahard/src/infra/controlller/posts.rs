use axum::{Json, extract::State, http::StatusCode};

use crate::{
    core::{
        application::posts::{dto::CreatePostInput, usecase::create_post::CreatePost},
        domain::posts::entities::Model,
    },
    error::AppError,
    state::AppState,
};

pub struct PostController;

impl PostController {
    pub async fn create(
        State(state): State<AppState>,
        Json(input): Json<CreatePostInput>,
    ) -> Result<(StatusCode, Json<Model>), AppError> {
        let post = CreatePost::new(state.posts).execute(input).await?;

        Ok((StatusCode::CREATED, Json(post)))
    }
}
