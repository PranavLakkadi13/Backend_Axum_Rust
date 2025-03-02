use crate::database::tasks::Entity as Tasks;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestSent {
    #[serde(
        default,  // important for deserialization
        skip_serializing_if = "Option::is_none", // important for serialization
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,  // important for deserialization
        skip_serializing_if = "Option::is_none", // important for serialization
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,  // important for deserialization
        skip_serializing_if = "Option::is_none", // important for serialization
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,  // important for deserialization
        skip_serializing_if = "Option::is_none", // important for serialization
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,  // important for deserialization
        skip_serializing_if = "Option::is_none", // important for serialization
        with = "::serde_with::rust::double_option"
    )]
    pub user_id: Option<Option<i32>>,
    #[serde(
        default,  // important for deserialization
        skip_serializing_if = "Option::is_none", // important for serialization
        with = "::serde_with::rust::double_option"
    )]
    pub is_default: Option<Option<bool>>,
}

// here the problem to deal is that if a set field is being updated or left blank because if no value is passed its none or
// even if a null value is passed its taken as none both cases cause the same issue so how to deal with the case when it is left blank and shouldn't
// be treated a null value rather a existing value not being updated
pub async fn partial_atomic_update_task(
    Extension(db_connection): Extension<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Json(body): Json<RequestSent>,
) -> Result<(), StatusCode> {
    println!("hit the partial update task route....");

    // getting the active model of a existing db task
    // here we are handing the case where if the id exists or not (if it doesn't we throw 404 error
    let mut get_task = if let Some(task) = Tasks::find_by_id(task_id)
        .one(&db_connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    println!("hello");

    // here we are dealing with the case where we will see if the data in db is actually being modified or not
    /*
    body.priority is Option<Option<String>>, so if let Some(priority) = body.priority:
    Case 1: None (Field missing)
        if let fails, and nothing happens.
        The field is not updated (expected behavior).
    Case 2: Some(None) (Field is explicitly null)
        if let succeeds, and priority is None.
        get_task.priority = Set(None) sets the column to NULL in the DB.
    Case 3: Some(Some(value)) (Field has a value)
        if let succeeds, and priority is Some(value).
        get_task.priority = Set(Some(value)) updates the DB with value.
    */
    if let Some(priority) = body.priority {
        get_task.priority = Set(priority)
    }

    if let Some(title) = body.title {
        get_task.title = Set(title)
    }

    if let Some(description) = body.description {
        get_task.description = Set(description)
    }

    if let Some(completed_at) = body.completed_at {
        get_task.completed_at = Set(completed_at)
    }

    if let Some(deleted_at) = body.deleted_at {
        get_task.deleted_at = Set(deleted_at)
    }

    // refer the sql code for the constraint logic -> users value being passed should exist in the users_table
    if let Some(user_id) = body.user_id {
        get_task.user_id = Set(user_id)
    }

    if let Some(is_default) = body.is_default {
        get_task.is_default = Set(is_default)
    }

    Tasks::update(get_task)
        .filter(<Tasks as EntityTrait>::Column::Id.eq(task_id))
        .exec(&db_connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
