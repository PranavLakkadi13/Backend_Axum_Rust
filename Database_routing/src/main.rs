use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use Database_routing::run;

#[tokio::main]
async fn main() {
    // here we are using the dotenv macro to set up and get the env variable
    dotenv().ok();
    let database_uri = dotenv!("DATABASE_URL");
    // the lib.rs file is actually running the code to connect to the db and the server routes
    run(database_uri).await // here we pass the db connection param which is parsed to the server handler
}
