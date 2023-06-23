use axum_todo_app::run;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL").to_owned(); // theres issue with rust-analyzer so we just say it is a string
    run(database_url).await;
}
