use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct MessageData {
    pub message: String,
    pub number: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MessageDataSentBack {
    pub message: String,
    pub response: String,
}

// here if you see the atg body is the message passed and has been type cast
pub async fn mirror_body_json_(Json(body): Json<MessageData>) -> Json<MessageDataSentBack> {
    println!("The debug value {:#?}", body);
    Json(MessageDataSentBack {
        message: body.message,
        response: format!("Response from Server {:?}", body.number),
    })
}
