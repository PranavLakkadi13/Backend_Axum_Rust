use axum::extract::Path;

// here the variables are typed in the path can be extracted using the Path extractor
pub async fn path_variables_fn(Path(id): Path<u32>) -> String {
    println!("the path variable request was successful");
    id.to_string()
}

pub async fn hard_coded_path() -> String {
    "You have hit the hardcode path 15".to_string()
}
