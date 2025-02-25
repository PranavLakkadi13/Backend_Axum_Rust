use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct MessageRequest {
    message: String,
    count: i32,
    username: String,
}
pub async fn get_json() -> Json<MessageRequest> {
    println!("hit the get_json route and got logged");
    Json(MessageRequest {
        message: "hello this is working".to_string(),
        count: 1,
        username: "damn boy got it working...".to_string(),
    })
}
