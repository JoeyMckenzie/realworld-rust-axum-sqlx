use std::future::ready;
use std::time::{Duration, Instant};

use anyhow::Context;
use axum::error_handling::HandleErrorLayer;
use axum::extract::MatchedPath;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{middleware, BoxError, Json, Router};
use clap::lazy_static::lazy_static;
use conduit_domain::PingResponse;
use http::{HeaderValue, Method, Request};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use conduit_infrastructure::service_register::ServiceRegister;

use crate::endpoints::articles_endpoints::ArticlesRouter;
use crate::endpoints::profiles_endpoints::ProfilesRouter;
use crate::endpoints::tags_endpoints::TagsRouter;
use crate::endpoints::users_endpoints::UsersRouter;

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
    static ref EXPONENTIAL_SECONDS: &'static [f64] = &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
}

pub struct ConduitApplicationController;

impl ConduitApplicationController {
    pub async fn serve(port: u32, cors_origin: &str, service_register: ServiceRegister) -> anyhow::Result<()> {
        let recorder_handle = PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Full(String::from("http_requests_duration_seconds")),
                *EXPONENTIAL_SECONDS,
            )
            .context("could not setup buckets for metrics, verify matchers are correct")?
            .install_recorder()
            .context("could not install metrics recorder")?;

        let router = Router::new()
            .nest("/api", UsersRouter::new_router(service_register.clone()))
            .nest("/api", ProfilesRouter::new_router(service_register.clone()))
            .nest("/api", ArticlesRouter::new_router(service_register.clone()))
            .nest("/api", TagsRouter::new_router(service_register))
            .route("/api/ping", get(Self::ping))
            .route("/metrics", get(move || ready(recorder_handle.render())))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(Self::handle_timeout_error))
                    .timeout(Duration::from_secs(*HTTP_TIMEOUT)),
            )
            .layer(
                CorsLayer::new()
                    .allow_origin(cors_origin.parse::<HeaderValue>().unwrap())
                    .allow_methods([Method::GET]),
            )
            .route_layer(middleware::from_fn(Self::track_metrics));

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
                Json(json!({ "error": format!("unhandled internal error: {}", err) })),
            )
        }
    }

    async fn track_metrics<B>(request: Request<B>, next: Next<B>) -> impl IntoResponse {
        let path = if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
            matched_path.as_str().to_owned()
        } else {
            request.uri().path().to_owned()
        };

        let start = Instant::now();
        let method = request.method().clone();
        let response = next.run(request).await;
        let latency = start.elapsed().as_secs_f64();
        let status = response.status().as_u16().to_string();

        let labels = [("method", method.to_string()), ("path", path), ("status", status)];

        metrics::increment_counter!("http_requests_total", &labels);
        metrics::histogram!("http_requests_duration_seconds", latency, &labels);

        response
    }

    async fn ping() -> Json<PingResponse> {
        info!("received ping request");
        Json(PingResponse::default())
    }
}
