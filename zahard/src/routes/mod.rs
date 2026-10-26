use axum::{
    Router,
    routing::{patch, post},
};

use crate::{
    infra::controlller::{
        BoardController, ImageController, PostController, PostReferenceController,
        ThreadController, UserController,
    },
    state::AppState,
};

pub struct AppChan;

impl AppChan {
    pub fn routes(state: AppState) -> Router {
        Router::new()
            .route("/boards", post(BoardController::create))
            .route("/images", post(ImageController::create))
            .route("/threads", post(ThreadController::create))
            .route("/posts", post(PostController::create))
            .route("/post-references", post(PostReferenceController::create))
            .route("/users", post(UserController::create))
            .route("/users/{id}", patch(UserController::update))
            .route("/users/login", post(UserController::login))
            .with_state(state)
    }
}
