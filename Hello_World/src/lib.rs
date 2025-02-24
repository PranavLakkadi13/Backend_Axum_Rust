use crate::routes::create_routes;

mod routes;

pub async fn run() {
    // here we will try to modularise it by getting the app impl into a new file
    let app = create_routes().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
