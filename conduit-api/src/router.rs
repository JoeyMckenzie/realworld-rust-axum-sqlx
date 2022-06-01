use crate::articles::build_articles_routes;
use crate::users::build_users_routes;
use anyhow::Context;
use axum::{Router};
use conduit_infrastructure::repositories::ConduitConnectionPool;
use conduit_utilities::config::AppConfig;

use std::sync::Arc;
use tracing::info;

pub async fn build_and_serve_api_router(
    config: AppConfig,
    pool: ConduitConnectionPool,
) -> anyhow::Result<()> {
    let port = config.port;
    let arc_pool = Arc::new(pool);

    let users_router = build_users_routes(arc_pool.clone());
    let articles_router = build_articles_routes(arc_pool.clone());
    let merged_router = users_router.merge(articles_router);
    let router = Router::new().nest("/api", merged_router);

    info!("Routes initialized! Now listening on port {}", port);
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(router.into_make_service())
        .await
        .context("error while starting API server");

    Ok(())
}
