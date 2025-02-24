use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct QueryReceived {
    message: String,
    id: u32,
}

pub async fn query_params(Query(data): Query<QueryReceived>) -> String {
    println!("The query route has been hit....");
    format!(
        "The data passed through query {0}, and {1}",
        data.message, data.id
    )
    .to_string()
}

pub async fn query_params_json(Query(data): Query<QueryReceived>) -> Json<QueryReceived> {
    println!("The query route has been hit and a json response sent.....");
    Json(data)
}
