use crate::database::tasks::Entity as Tasks;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Extension;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use serde_with::serde_derive::Deserialize;

// here we are on the assumption that the whole task is deleted
pub async fn delete_task(
    Path(task_id): Path<i32>,
    Extension(db_connection): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    println!("hit the delete_task route..");
    // let task = if let Some(task) = <Tasks as EntityTrait>::find_by_id(task_id)
    //     .one(&db_connection)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    // {
    //     task.into_active_model()
    // } else {
    //     return Err(StatusCode::NOT_FOUND);
    // };
    //
    // Tasks::delete(task)
    //     .exec(&db_connection)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // In the above code we were doing is checking if the value exists and then deleting
    // the below is a shorter way to click delete
    // Tasks::delete_by_id(task_id)
    //     .exec(&db_connection)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // below is the code in the case when it is a multiple delete
    Tasks::delete_many()
        .filter(<Tasks as EntityTrait>::Column::Id.gt(task_id)) // if didn't use the filter it would have deleted everything in the db
        .exec(&db_connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub soft: bool,
}

pub async fn soft_delete(
    Path(task_id): Path<i32>,
    Extension(db_connection): Extension<DatabaseConnection>,
    Query(params): Query<QueryParams>,
) -> Result<(), StatusCode> {
    println!("hit the soft delete route....");

    // in partial delete we just add the deleted_at column value to a timestamp
    if params.soft == true {
        let mut task = if let Some(task) = <Tasks as EntityTrait>::find_by_id(task_id)
            .one(&db_connection)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            task.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

        let now = chrono::Utc::now();

        task.deleted_at = Set(Some(now.into()));

        Tasks::update(task)
            .exec(&db_connection)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Tasks::delete_many()
            .filter(<Tasks as EntityTrait>::Column::Id.gt(task_id)) // if didn't use the filter it would have deleted everything in the db
            .exec(&db_connection)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(())
}
