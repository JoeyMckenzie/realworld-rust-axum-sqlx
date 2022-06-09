use axum::routing::get;
use axum::{routing::post, Extension, Router};

use conduit_core::users::service::DynUsersService;

use crate::users::endpoints::{
    get_current_user_endpoint, login_user_endpoint, register_user_endpoint,
};

pub mod endpoints;

pub struct UsersRouter;

impl UsersRouter {
    pub fn new_router(users_service: DynUsersService) -> Router {
        Router::new()
            .route("/users", post(register_user_endpoint))
            .route("/users/login", post(login_user_endpoint))
            .route("/user", get(get_current_user_endpoint))
            .layer(Extension(users_service))
    }
}
