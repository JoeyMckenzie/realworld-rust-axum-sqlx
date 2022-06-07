use anyhow::Context;

use clap::Parser;
use conduit_api::router::build_and_serve_api_router;
use conduit_infrastructure::repositories::initialize_pg_pool_with_migrations;
use conduit_infrastructure::service_register::ServiceRegister;
use conduit_utilities::config::AppConfig;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Build out the logging instance using tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Initialize environment variables
    info!("loading environment variables...");
    dotenv::dotenv().ok();

    info!("environment loaded, parsing configuration...");
    let config = AppConfig::parse();

    info!("configuration parsed, initializing Postgres connection and running migrations...");
    let pg_pool = initialize_pg_pool_with_migrations(&config.database_url)
        .await
        .expect("could not initialize the database connection pool");

    let service_register = ServiceRegister::new(pg_pool);

    info!("migrations successfully ran, initializing axum server...");
    build_and_serve_api_router(config, service_register)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
