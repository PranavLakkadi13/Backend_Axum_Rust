use axum::http::StatusCode;

/*
* the 200 - 299 series of status code is when everything is working fine
* the 300 - 399 series is for small issues but works case (like redirection)
* the 400 - 499 series is for client side error and the client messed up so no response
    -> 401 - 403 is auth based
* the 500 - 599 series is for the server that the server internally did something wrong and it crashed
*/
pub async fn always_errors() -> Result<(), StatusCode> {
    println!("hit the custom error route...");
    Err(StatusCode::IM_A_TEAPOT)
}
