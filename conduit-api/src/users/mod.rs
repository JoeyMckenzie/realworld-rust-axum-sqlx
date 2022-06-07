use crate::users::endpoints::register_user_endpoint;
use axum::{routing::post, Extension, Router};
use conduit_core::users::DynUsersService;

pub mod endpoints;

pub struct UsersController;

impl UsersController {
    pub fn new_router(users_service: DynUsersService) -> Router {
        Router::new()
            .route("/users", post(register_user_endpoint))
            .layer(Extension(users_service))
    }
}
