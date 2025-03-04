use crate::database::users;
use crate::database::users::Entity as Users;
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use serde::Serialize;
use serde_with::serde_derive::Deserialize;
#[derive(Deserialize)]
pub struct UserRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    username: String,
    id: i32,
    token: String,
}
pub async fn create_user(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(body): Json<UserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    println!("Hit the create_user route....");
    println!("{:?}", body.password);
    println!("{:?}", body.username);

    let new_user = users::ActiveModel {
        username: Set(body.username),
        password: Set(body.password),
        token: Set(Some("ehabfcj-kbsdckj-bsdc".to_string())),
        ..Default::default()
    }
    .save(&db_connection)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(UserResponse {
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

pub async fn login(
    Extension(db_connection): Extension<DatabaseConnection>,
    Json(body): Json<UserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    println!("Hit the login route...");

    // remember that to interact with the database you always need an entity
    let mut task = Users::find()
        // basically I am searching the db based on the column top look for the required username
        .filter(<Users as EntityTrait>::Column::Username.eq(body.username)) // also gets the tasks that are not null in delete column
        .one(&db_connection)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(task) = task {
        // doing the login part here

        // because here we are assuming that the token exists but if it doesn't we would want to create one
        // also we would want to make a new token at each login instance
        // Ok(Json(UserResponse {
        //     username: task.username,
        //     id: task.id,
        //     token: task.token,
        // }))

        // once we know the username exists we create a new token
        let new_token = "2734726437673842".to_string();
        let mut active_task = task.into_active_model();


        active_task.token = Set(Some(new_token));

        let saved_user = active_task
            .save(&db_connection)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(UserResponse {
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
