use anyhow::Context;

use clap::Parser;
use conduit_api::router::build_and_serve_api_router;
use conduit_infrastructure::repositories::initialize_pg_pool;
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
    info!("Loading environment variables...");
    dotenv::dotenv().ok();

    let config = AppConfig::parse();

    // Spin up a connection to the database and run any outstanding migrations
    info!("Initializing Postgres connection and running migrations...");
    let pg_pool = initialize_pg_pool(&config.database_url)
        .await
        .context("could not initialize the database connection pool");

    info!("Migrations successfully ran! Initializing axum server...");
    build_and_serve_api_router(config, pg_pool.unwrap())
        .await
        .context("could not initialize application routes");

    Ok(())
}
