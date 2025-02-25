use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn returns_201() -> Response {
    (StatusCode::CREATED, ()).into_response() // this is the code for 201
}
