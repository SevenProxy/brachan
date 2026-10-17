use std::sync::Arc;

use crate::core::domain::{
    boards::repositories::BoardsRepository, images::repositories::ImagesRepository,
    post_references::repositories::PostReferencesRepository, posts::repositories::PostsRepository,
    threads::repositories::ThreadsRepository, users::repositories::UsersRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub boards: Arc<dyn BoardsRepository + Send + Sync>,
    pub images: Arc<dyn ImagesRepository + Send + Sync>,
    pub threads: Arc<dyn ThreadsRepository + Send + Sync>,
    pub posts: Arc<dyn PostsRepository + Send + Sync>,
    pub post_references: Arc<dyn PostReferencesRepository + Send + Sync>,
    pub users: Arc<dyn UsersRepository + Send + Sync>,
}
