#![allow(dead_code)]

mod core;
mod error;
mod infra;
mod routes;
mod state;

use std::sync::Arc;

use core::application::users::usecase::bootstrap_admin::BootstrapAdmin;
use infra::database::repository::{
    boards_repository::BoardsRepositoryImpl, images_repository::ImagesRepositoryImpl,
    post_references_repository::PostReferencesRepositoryImpl,
    posts_repository::PostsRepositoryImpl, threads_repository::ThreadsRepositoryImpl,
    users_repository::UsersRepositoryImpl,
};
use routes::AppChan;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set (see .env.example)");
    let db = infra::database::connect(&database_url).await?;

    let state = AppState {
        boards: Arc::new(BoardsRepositoryImpl::new(db.clone())),
        images: Arc::new(ImagesRepositoryImpl::new(db.clone())),
        threads: Arc::new(ThreadsRepositoryImpl::new(db.clone())),
        posts: Arc::new(PostsRepositoryImpl::new(db.clone())),
        post_references: Arc::new(PostReferencesRepositoryImpl::new(db.clone())),
        users: Arc::new(UsersRepositoryImpl::new(db.clone())),
    };

    if let Some(admin) = BootstrapAdmin::new(state.users.clone()).execute().await? {
        tracing::info!("bootstrap: initial admin '{}' created", admin.username);
    }

    let app = AppChan::routes(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
