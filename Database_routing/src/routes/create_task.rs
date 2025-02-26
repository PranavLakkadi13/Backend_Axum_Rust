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
    // so firstly we need to get the active model to then start updating the db
    let new_task = tasks::ActiveModel {
        priority: Set(body.priority),
        title: Set(body.title),
        description: Set(body.description),
        ..Default::default() // this is to basically say that the not selected values have a default value set
    };

    // this save returns a active model which can further be modified to save data
    let result = new_task.save(&db_connect).await.ok().unwrap();

    println!("logged the new task....");
    dbg!(result);
}
