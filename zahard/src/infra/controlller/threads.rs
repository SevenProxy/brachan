use axum::{Json, extract::State, http::StatusCode};

use crate::{
    core::{
        application::threads::{dto::CreateThreadInput, usecase::create_thread::CreateThread},
        domain::threads::entities::Model,
    },
    error::AppError,
    state::AppState,
};

pub struct ThreadController;

impl ThreadController {
    pub async fn create(
        State(state): State<AppState>,
        Json(input): Json<CreateThreadInput>,
    ) -> Result<(StatusCode, Json<Model>), AppError> {
        let thread = CreateThread::new(state.threads).execute(input).await?;

        Ok((StatusCode::CREATED, Json(thread)))
    }
}
