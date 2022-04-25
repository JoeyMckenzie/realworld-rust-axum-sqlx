use axum::{routing::get, Router};

pub fn build_users_routes() -> Router {
    Router::new().route("/users", get(|| async { "hello! " }))
}
