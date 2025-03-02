use crate::database::tasks::Entity as Tasks; // here we are using the orm code and getting it as Entity
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

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
    } else {
        println!("the task doesn't exist and handling error");
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
pub struct QueryParams {
    priority: Option<String>,
}

pub async fn get_all_tasks(
    Extension(db_connection): Extension<DatabaseConnection>,
    Query(query): Query<QueryParams>,
) -> Result<Json<Vec<ResponseClient>>, StatusCode> {
    println!("hit the all_tasks route with a filter....");

    // here it is used to pass the query filter
    let mut priority_filter = Condition::all();
    if let Some(priority) = query.priority {
        priority_filter = if priority.is_empty() {
            priority_filter.add(<Tasks as EntityTrait>::Column::Priority.is_null())
        // to match query where the priority is null
        } else {
            priority_filter.add(<Tasks as EntityTrait>::Column::Priority.eq(priority))
        }
    }

    // after the doing the below operation it doesn't know what type to get it into
    // so after doing a map and collect when in the bottom we see the all_tasks being passed to the Json the above Json<ResponseClient> type is inferred
    let all_tasks = Tasks::find()
        // the below method issue is that if the optional filter is like none it won't know how to deal with it
        // .filter(<Tasks as EntityTrait>::Column::Priority.eq(query.priority)) // here we are keeping the filter based on the query filter passed
        .filter(priority_filter.clone()) // this is using the priority filter
        .all(&db_connection) // here we are getting all the tasks instead of just 1
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)? // here we are mapping any error that may occur to a status code
        .into_iter() // converting into iterator so be passed through
        .map(|db_task| ResponseClient {
            // passing each model type to the result struct that we want
            id: db_task.id,
            title: db_task.title,
            description: db_task.description,
            priority: db_task.priority,
        })
        .collect();

    Ok(Json(all_tasks))
}
