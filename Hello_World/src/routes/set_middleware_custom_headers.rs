use crate::routes::read_middleware_custom_headers::HeaderMessage;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;

// the main goal was to grab the data from the headers and to parse it as a request and move to the next handler
pub async fn set_middleware_custom_headers(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_error| StatusCode::BAD_REQUEST)?
        .to_string();
    req.extensions_mut()
        .insert(HeaderMessage(message.to_string()));
    Ok(next.run(req).await)
}
