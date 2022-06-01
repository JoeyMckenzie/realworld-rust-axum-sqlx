use crate::users::endpoints::register_user_endpoint;
use axum::{routing::post, Extension, Router};
use conduit_core::users::repository::DynUsersRepository;
use conduit_core::users::service::DynUserService;
use conduit_infrastructure::repositories::user_repository::UsersRepositoryImpl;
use conduit_infrastructure::repositories::ConduitConnectionPool;
use conduit_infrastructure::services::users_service::UsersServiceImpl;
use std::sync::Arc;

pub mod endpoints;

pub fn build_articles_routes(pool: Arc<ConduitConnectionPool>) -> Router {
    let users_repository = Arc::new(UsersRepositoryImpl::new(pool)) as DynUsersRepository;
    let users_service = Arc::new(UsersServiceImpl::new(users_repository)) as DynUserService;

    Router::new()
        .route("/articles", post(register_user_endpoint))
        .layer(Extension(users_service))
}
