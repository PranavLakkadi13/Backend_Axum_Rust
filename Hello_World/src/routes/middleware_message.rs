use crate::routes::SharedData;
use axum::Extension;

pub async fn middleware_message(Extension(data): Extension<SharedData>) -> String {
    println!("the shared message route is hit....");
    data.message.to_string()
}
