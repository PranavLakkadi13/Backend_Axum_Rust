use crate::database::tasks::Entity as Tasks; // here we are using the orm code and getting it as Entity
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

// this is the struct data that we will be sending back to the client and not the whole db struct
#[derive(Serialize)]
pub struct ResponseClient {
    id: i32,
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

pub async fn get_task(
    Path(task_id): Path<i32>,
    Extension(db_connection): Extension<DatabaseConnection>,
) -> Result<Json<ResponseClient>, StatusCode> {
    let task = Tasks::find_by_id(task_id)
        .one(&db_connection)
        .await
        .ok()
        .unwrap();

    println!("hit the get_task route.....");
    if let Some(task) = task {
        println!("The task exist and being sent to client");
        Ok(Json(ResponseClient {
            id: task.id,
            title: task.title,
            description: task.description,
            priority: task.priority,
        }))
    }
    else {
        println!("the task doesn't exist and handling error");
        Err(StatusCode::NOT_FOUND)
    }
}
