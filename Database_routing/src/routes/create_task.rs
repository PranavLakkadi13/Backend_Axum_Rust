use crate::database::tasks;
use axum::{Extension, Json};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

pub async fn create_task(
    Extension(db_connect): Extension<DatabaseConnection>,
    Json(body): Json<RequestTask>,
) {
    // since it is optional field wanted to populate it with null value
    let priority_value = if body.priority.clone().unwrap().is_empty() {
        None
    } else {
        body.priority
    };

    let description_value = if body.description.clone().unwrap().is_empty() {
        None
    } else {
        body.description
    };

    // so firstly we need to get the active model to then start updating the db
    let new_task = tasks::ActiveModel {
        priority: Set(priority_value),
        title: Set(body.title),
        description: Set(description_value),
        ..Default::default() // this is to basically say that the not selected values have a default value set
    };

    // this save returns an active model which can further be modified to save data
    let result = new_task.save(&db_connect).await.ok().unwrap();

    println!("logged the new task....");
    dbg!(result);
}
