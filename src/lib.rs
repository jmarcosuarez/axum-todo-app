use router::create_router;

mod router;
mod routes;

pub async fn run() {
    let app = create_router();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
