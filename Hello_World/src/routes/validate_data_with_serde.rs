use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    pub username: String,
}

// the main goal was to see what happens when we pass a json
// and when we pass a missing value we will get an error since it was expecting a value but couldn't find one
// if the struct had an Option<String> then it would have passed
pub async fn validate_data_with_serde(Json(req): Json<RequestUser>) {
    dbg!(req);
}
