use axum_todo_app::{app_state::AppState, run, utils::token_wrapper::TokenWrapper};
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL").to_owned(); // theres issue with rust-analyzer so we just say it is a string
    let jwt_secret = dotenv!("JWT_SECRET").to_owned();
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };
    let app_state = AppState {
        db,
        jwt_secret: TokenWrapper(jwt_secret),
    };

    run(app_state).await;
}
