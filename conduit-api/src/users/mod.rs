use crate::users::endpoints::register_user_endpoint;
use axum::{routing::post, Extension, Router};
use conduit_core::users::service::DynUsersService;

pub mod endpoints;

pub fn build_users_routes(users_service: DynUsersService) -> Router {
    Router::new()
        .route("/users", post(register_user_endpoint))
        .layer(Extension(users_service))
}
