use axum::Router;

use crate::lib::users::router::build_users_routes;

pub fn build_api_router() -> Router {
    Router::new()
        .nest("/api", build_users_routes())
}