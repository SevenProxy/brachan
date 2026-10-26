use axum::{Json, extract::State, http::StatusCode};

use crate::{
    core::{
        application::users::{
            dto::{UpdateUserInput, UserInput},
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
        Json(input): Json<UpdateUserInput>,
    ) -> Result<Json<Model>, AppError> {
        let update = UpdateUser::new(state.users)
            .execute(input.id, input.update)
            .await?;

        Ok(Json(update))
    }
}
