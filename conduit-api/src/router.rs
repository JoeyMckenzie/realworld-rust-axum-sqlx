use crate::users::build_users_routes;
use anyhow::Context;
use axum::Router;
use conduit_utilities::config::AppConfig;
use tracing::info;
use conduit_infrastructure::service_register::ServiceRegister;

pub async fn build_and_serve_api_router(
    config: AppConfig,
    service_register: ServiceRegister,
) -> anyhow::Result<()> {
    let port = config.port;

    let users_router = build_users_routes(service_register.users_service);
    let router = Router::new().nest("/api", users_router);

    info!("routes initialized, listening on port {}", port);
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse().unwrap())
        .serve(router.into_make_service())
        .await
        .context("error while starting API server")?;

    Ok(())
}
