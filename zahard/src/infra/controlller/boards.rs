use axum::{Json, extract::State, http::StatusCode};

use crate::{
    core::{
        application::boards::{dto::CreateBoardInput, usecase::create_board::CreateBoard},
        domain::boards::entities::Model,
    },
    error::AppError,
    state::AppState,
};

pub struct BoardController;

impl BoardController {
    pub async fn create(
        State(state): State<AppState>,
        Json(input): Json<CreateBoardInput>,
    ) -> Result<(StatusCode, Json<Model>), AppError> {
        let board = CreateBoard::new(state.boards).execute(input).await?;

        Ok((StatusCode::CREATED, Json(board)))
    }
}
