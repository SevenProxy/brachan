mod infra;
mod routes;

use routes::AppChan;

#[tokio::main]
async fn main() {
    let app = AppChan::routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
