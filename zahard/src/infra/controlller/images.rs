use axum::{Json, extract::State, http::StatusCode};

use crate::{
    core::{
        application::images::{dto::CreateImageInput, usecase::create_image::CreateImage},
        domain::images::entities::Model,
    },
    error::AppError,
    state::AppState,
};

pub struct ImageController;

impl ImageController {
    pub async fn create(
        State(state): State<AppState>,
        Json(input): Json<CreateImageInput>,
    ) -> Result<(StatusCode, Json<Model>), AppError> {
        let image = CreateImage::new(state.images).execute(input).await?;

        Ok((StatusCode::CREATED, Json(image)))
    }
}
