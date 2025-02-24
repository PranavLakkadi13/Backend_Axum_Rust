mod hello_world_handler;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod path_variables;
mod query_params;

// use axum::body::Body;
use axum::routing::{get, post};
use axum::Router;
use hello_world_handler::hello_world;
use mirror_body_json::mirror_body_json_;
use mirror_body_string::mirror_body;
use mirror_custom_header::mirror_custom_error;
use path_variables::{hard_coded_path, path_variables_fn};
use query_params::{query_params, query_params_json};

pub async fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body))
        .route("/mirror_body_json", post(mirror_body_json_))
        // here the ordering doesn't matter for the path_variables route since it will the most effective one
        .route("/path_variables/151", get(hard_coded_path))
        .route("/path_variables/{id}", get(path_variables_fn))
        .route("/query_params", get(query_params))
        .route("/query_params_json", get(query_params_json))
        .route("/custom_header", get(mirror_custom_error))
}
