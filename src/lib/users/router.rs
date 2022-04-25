use axum::{
    Router,
    routing::get
};

pub fn build_users_routes() -> Router {
    Router::new()
        .route("/users", get(|| async { "hello! "}))
}