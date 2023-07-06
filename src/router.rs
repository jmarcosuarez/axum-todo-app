use crate::{
    app_state::AppState,
    routes::{
        hello_world::hello_world,
        users::{create_user::create_user, login::login, logout::logout},
    },
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/hello_world", get(hello_world))
        .route("/api/v1/users", post(create_user))
        .route("/api/v1/users/login", post(login))
        .route("/api/v1/users/logout", post(logout))
        .with_state(app_state)
}
