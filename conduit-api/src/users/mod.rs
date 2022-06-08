use crate::users::endpoints::register_user_endpoint;
use axum::{routing::post, Extension, Router};
use conduit_core::users::DynUsersService;
use conduit_utilities::config::AppConfig;
use std::sync::Arc;

pub mod endpoints;

pub struct UsersController;

impl UsersController {
    pub fn new_router(config: Arc<AppConfig>, users_service: DynUsersService) -> Router {
        Router::new()
            .route("/users", post(register_user_endpoint))
            .layer(Extension(users_service))
            .layer(Extension(config))
    }
}
