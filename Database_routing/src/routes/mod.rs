use axum::http::Method;
use axum::routing::{get, post};
use axum::{Extension, Router};
use sea_orm::DatabaseConnection;

mod create_task;
mod get_task;
mod hello_world;

use create_task::create_task;
use get_task::get_task;
use hello_world::hello_world;

pub async fn create_routes(database_url: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/create_task", post(create_task))
        .route("/get_task/{task_id}", get(get_task))
        // created a middleware layer for the database to be accessed
        // and as you know when the middleware is declared all the routes above it can access the middleware
        .layer(Extension(database_url))
}
