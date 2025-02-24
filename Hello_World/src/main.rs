// using the below login since I moved the router code to the lib file and to access the fn I used it
// because it was declared in lib we use the Package name from Cargo.toml
use Hello_World::run;

#[tokio::main]
async fn main() {
    run().await
}
