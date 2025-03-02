use crate::database::tasks;
use crate::database::tasks::Entity as Tasks;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}

// here the user will update the whole thing and if he just modifies 2 fields and leaves the rest as blank they will be set as null
pub async fn atomic_update_task(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Json(body): Json<RequestTask>,
) -> Result<(), StatusCode> {
    println!("hit the update task route....");

    let updated_task = tasks::ActiveModel {
        id: Set(task_id),
        priority: Set(body.priority),
        title: Set(body.title),
        completed_at: Set(body.completed_at),
        description: Set(body.description),
        deleted_at: Set(body.deleted_at),
        user_id: Set(body.user_id),
        is_default: Set(body.is_default),
    };

    // here we are updating the entity with the specific id else without filter it would have updated the entire db
    Tasks::update(updated_task)
        .filter(<Tasks as EntityTrait>::Column::Id.eq(task_id))
        .exec(&db_connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
