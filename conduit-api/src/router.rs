use std::time::Duration;

use anyhow::Context;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::{BoxError, Json, Router};
use clap::lazy_static::lazy_static;
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use conduit_infrastructure::service_register::ServiceRegister;

use crate::endpoints::articles_endpoints::ArticlesRouter;
use crate::endpoints::profiles_endpoints::ProfilesRouter;
use crate::endpoints::tags_endpoints::TagsRouter;
use crate::endpoints::users_endpoints::UsersRouter;

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
}

pub struct ConduitApplicationController;

impl ConduitApplicationController {
    pub async fn serve(port: u32, service_register: ServiceRegister) -> anyhow::Result<()> {
        let router = Router::new()
            .nest("/api", UsersRouter::new_router(service_register.clone()))
            .nest("/api", ProfilesRouter::new_router(service_register.clone()))
            .nest("/api", ArticlesRouter::new_router(service_register.clone()))
            .nest("/api", TagsRouter::new_router(service_register))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(Self::handle_timeout_error))
                    .timeout(Duration::from_secs(*HTTP_TIMEOUT)),
            );

        info!("routes initialized, listening on port {}", port);
        axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
            .serve(router.into_make_service())
            .await
            .context("error while starting API server")?;

        Ok(())
    }

    /// Adds a custom handler for tower's `TimeoutLayer`, see https://docs.rs/axum/latest/axum/middleware/index.html#commonly-used-middleware.
    async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
        if err.is::<tower::timeout::error::Elapsed>() {
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({
                    "error":
                        format!(
                            "request took longer than the configured {} second timeout",
                            *HTTP_TIMEOUT
                        )
                })),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("unhandled internal error: {}", err)
                })),
            )
        }
    }
}
