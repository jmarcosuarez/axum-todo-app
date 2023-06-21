use crate::routes::hello_world::hello_world;
use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new().route("/hello_world", get(hello_world))
}
