use crate::users::router::build_users_routes;
use axum::{Extension, Router};
use conduit_shared::context::SyncAppContext;

pub fn build_api_router(context: SyncAppContext) -> Router {
    Router::new()
        .nest("/api", build_users_routes())
        .layer(Extension(context))
}
