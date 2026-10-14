#![allow(dead_code)]

mod core;
mod error;
mod infra;
mod routes;

use std::sync::Arc;

use core::application::users::usecase::bootstrap_admin::BootstrapAdmin;
use infra::database::repository::users_repository::UsersRepositoryImpl;
use routes::AppChan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set (see .env.example)");
    let db = infra::database::connect(&database_url).await?;

    let users_repository = Arc::new(UsersRepositoryImpl::new(db.clone()));
    if let Some(admin) = BootstrapAdmin::new(users_repository).execute().await? {
        tracing::info!("bootstrap: initial admin '{}' created", admin.username);
    }

    let app = AppChan::routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
