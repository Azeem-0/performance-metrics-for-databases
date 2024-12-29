use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use db::init_databases;
use handlers::{fetch_and_insert_data::fetch_and_insert_data, read_data::read_data};

mod db;
mod handlers;
mod metrics;
mod models;
mod utils;

async fn get_router() -> Router {
    let database = init_databases().await.map_err(|e| e).unwrap();

    let shared_database = Arc::new(database);

    Router::new()
        .route("/", get(root))
        .route("/fetch-and-insert-data", get(fetch_and_insert_data))
        .route("/read-data", get(read_data))
        .layer(Extension(shared_database))
}

#[tokio::main]
async fn main() {
    let app = get_router().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
async fn root() -> &'static str {
    "Hello, Here we go again!"
}
