use axum::http::HeaderMap;

// this helps get the http headers
pub async fn mirror_custom_header(header_name: HeaderMap) -> String {
    println!("hit the custom header route...");
    let message = header_name.get("Content-Length").unwrap();
    message.to_str().unwrap().to_string()
}
