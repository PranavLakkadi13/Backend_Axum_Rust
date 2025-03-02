use axum::routing::{delete, get, patch, post, put};
use axum::{Extension, Router};
use sea_orm::DatabaseConnection;

mod create_task;
mod delete_task;
mod get_task;
mod hello_world;
mod partial_update_task;
mod update_task;

use create_task::create_task;
use delete_task::{delete_task, soft_delete};
use get_task::{get_all_tasks, get_task};
use hello_world::hello_world;
use partial_update_task::partial_atomic_update_task;
use update_task::atomic_update_task;

pub async fn create_routes(database_url: DatabaseConnection) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/create_task", post(create_task))
        .route("/get_task/{task_id}", get(get_task))
        .route("/get_task", get(get_all_tasks))
        // the main understanding of using a put is that it is used to update the existing data as a whole and not partially
        .route("/atomic_update/{task_id}", put(atomic_update_task))
        // here I am doing this patch to partially update the tasks like a specific column or a row
        .route(
            "/partial_update/{task_id}",
            patch(partial_atomic_update_task),
        )
        .route("/delete_task/{task_id}", delete(delete_task))
        .route("/delete_task/{task_id}", put(soft_delete))
        // created a middleware layer for the database to be accessed
        // and as you know when the middleware is declared all the routes above it can access the middleware
        .layer(Extension(database_url))
}
