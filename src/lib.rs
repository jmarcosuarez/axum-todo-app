use router::create_router;
use sea_orm::Database;
use utils::app_state::AppState;

mod database;
mod router;
mod routes;
mod utils;

pub async fn run(database_url: String) {
    let db = match Database::connect(database_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };
    let app_state = AppState { db };
    let app = create_router(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
