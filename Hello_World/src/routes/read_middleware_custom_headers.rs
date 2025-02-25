use axum::Extension;

// this is called a tuple struct
#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn read_middleware_custom_headers(
    Extension(message): Extension<HeaderMessage>,
) -> String {
    println!("hit the custom read route.... ");
    println!("the message received is {:?}", message.0.to_string());
    message.0.to_string()
}
