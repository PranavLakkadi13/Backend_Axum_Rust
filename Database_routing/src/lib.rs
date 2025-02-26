mod database;
mod routes;

use routes::create_routes;
use sea_orm::Database;
use tokio::net::TcpListener;

pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    // here we pass the database instance to the serve to use it
    let app = create_routes(database).await;
    let listener = TcpListener::bind(("0.0.0.0:3000")).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
