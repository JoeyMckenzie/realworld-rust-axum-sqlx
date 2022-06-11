use crate::profiles::endpoints::get_profile;
use axum::routing::get;
use axum::{Extension, Router};
use conduit_infrastructure::service_register::ServiceRegister;

pub mod endpoints;

pub struct ProfilesRouter;

impl ProfilesRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/profiles/:username", get(get_profile))
            .layer(Extension(service_register.profiles_service))
            .layer(Extension(service_register.token_service))
    }
}
