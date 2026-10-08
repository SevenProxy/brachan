use crate::infra::controlller::BoardController;

use axum::{Router, routing::get};

pub struct AppChan;

impl AppChan {
    pub fn routes() -> Router {
        Router::new().route("/create-board", get(BoardController::create))
    }
}
