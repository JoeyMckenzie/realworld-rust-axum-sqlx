use crate::users::UsersController;
use anyhow::Context;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::{BoxError, Json, Router};
use conduit_core::infrastructure::service_register::ServiceRegister;
use conduit_utilities::config::AppConfig;
use serde_json::json;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

static HTTP_TIMEOUT: u64 = 30;

pub struct ConduitApplicationController;

impl ConduitApplicationController {
    pub async fn serve(config: AppConfig, service_register: ServiceRegister) -> anyhow::Result<()> {
        let port = config.port;

        let router = Router::new()
            .nest(
                "/api",
                UsersController::new_router(service_register.users_service),
            )
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(Self::handle_timeout_error))
                    .timeout(Duration::from_secs(HTTP_TIMEOUT)),
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
                            HTTP_TIMEOUT
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
