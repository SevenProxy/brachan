use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    core::{
        application::users::{
            dto::{UpdateUserApplication, UserInput},
            usecase::{create_user::CreateUser, login_user::LoginUser, update_user::UpdateUser},
        },
        domain::users::entities::Model,
    },
    error::AppError,
    state::AppState,
};

pub struct UserController;

impl UserController {
    pub async fn create(
        State(state): State<AppState>,
        Json(input): Json<UserInput>,
    ) -> Result<(StatusCode, Json<Model>), AppError> {
        let user = CreateUser::new(state.users).execute(input).await?;

        Ok((StatusCode::CREATED, Json(user)))
    }

    pub async fn login(
        State(state): State<AppState>,
        Json(input): Json<UserInput>,
    ) -> Result<Json<String>, AppError> {
        let token = LoginUser::new(state.users).execute(input).await?;

        Ok(Json(token))
    }

    pub async fn update(
        State(state): State<AppState>,
        Path(id): Path<i64>,
        Json(input): Json<UpdateUserApplication>,
    ) -> Result<Json<Model>, AppError> {
        let user = UpdateUser::new(state.users).execute(id, input).await?;

        Ok(Json(user))
    }
}
