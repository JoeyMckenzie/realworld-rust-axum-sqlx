use crate::profiles::endpoints::{follow_user, get_profile, unfollow_user};
use axum::routing::{delete, get, post};
use axum::{Extension, Router};
use conduit_infrastructure::service_register::ServiceRegister;

pub mod endpoints;

pub struct ProfilesRouter;

impl ProfilesRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/profiles/:username", get(get_profile))
            .route("/profiles/:username/follow", post(follow_user))
            .route("/profiles/:username/follow", delete(unfollow_user))
            .layer(Extension(service_register.profiles_service))
            .layer(Extension(service_register.token_service))
    }
}
