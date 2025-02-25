mod always_errors;
mod custom_json_extractor;
mod get_json;
mod hello_world_handler;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod path_variables;
mod query_params;
mod read_middleware_custom_headers;
mod returns_201;
mod set_middleware_custom_headers;
mod validate_data_with_serde;

use always_errors::always_errors;
use axum::http::Method;
use axum::routing::{get, post};
use axum::{Extension, Router};
use custom_json_extractor::custom_json_extractor;
use get_json::get_json;
use hello_world_handler::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json_;
use mirror_body_string::mirror_body;
use mirror_custom_header::mirror_custom_header;
use path_variables::{hard_coded_path, path_variables_fn};
use query_params::{query_params, query_params_json};
use read_middleware_custom_headers::read_middleware_custom_headers;
use returns_201::returns_201;
use set_middleware_custom_headers::set_middleware_custom_headers;
use tower_http::cors::{Any, CorsLayer};
use validate_data_with_serde::validate_data_with_serde;

#[derive(Clone)]
// this is basically for the custom middleware to access the shared data as a layer
pub struct SharedData {
    message: String,
}

pub async fn create_routes() -> Router {
    // this is used as a middleware for access control
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_string(),
    };

    Router::new()
        // wrote these 2 separate because the first route will have a custom middleware in the form of the route_layer handler
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_headers),
        ) // note this function will not have the above layer code in them as middleware
        .route_layer(axum::middleware::from_fn(set_middleware_custom_headers)) // now this custom middleware will run for all instead of just the above route
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body))
        .route("/mirror_body_json", post(mirror_body_json_))
        // here the ordering doesn't matter for the path_variables route since it will the most effective one
        .route("/path_variables/151", get(hard_coded_path))
        .route("/path_variables/{id}", get(path_variables_fn))
        .route("/query_params", get(query_params))
        .route("/query_params_json", get(query_params_json))
        .route("/custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(cors) // here we have added the middleware also this middleware will be triggered when any of the above routes are hit
        .layer(Extension(shared_data)) // the layer position matters a lot since all the paths have to access it then it has to be in the bottom
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(validate_data_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
}
