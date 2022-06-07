use anyhow::Context;
use clap::Parser;
use conduit_api::router::ConduitApplicationController;
use conduit_core::infrastructure::connection_pool::ConduitConnectionManager;
use conduit_core::infrastructure::service_register::ServiceRegister;
use conduit_utilities::config::AppConfig;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Build out the logging instance using tracing
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::DEBUG)
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Initialize environment variables
    dotenv::dotenv().ok();

    let config = AppConfig::parse();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let pg_pool =
        ConduitConnectionManager::new_pg_pool(&config.database_url, config.run_migrations)
            .await
            .expect("could not initialize the database connection pool");

    let service_register = ServiceRegister::new(pg_pool);

    info!("migrations successfully ran, initializing axum server...");
    ConduitApplicationController::serve(config, service_register)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
