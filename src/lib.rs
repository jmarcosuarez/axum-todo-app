use app_state::AppState;
use router::create_router;

pub mod app_state;
mod database;
mod middleware;
mod router;
mod routes;
pub mod utils;

pub async fn run(app_state: AppState) {
    let app = create_router(app_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
