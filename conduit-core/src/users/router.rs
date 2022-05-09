use crate::users::service::register_user;
use axum::{routing::post, Router};

pub fn build_users_routes() -> Router {
    Router::new().route("/users", post(register_user))
}
