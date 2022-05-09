use std::env;

use conduit_core::routes::build_api_router;
use conduit_repository::initialize_db;

use conduit_shared::context::AppContext;
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

    let connection_string =
        env::var("CONNECTION_STRING").expect("connection string is not available");

    // Spin up a connection to the database and run any outstanding migrations
    info!("Running migrations...");
    let db = initialize_db(&connection_string).await;

    info!("Migrations successfully ran! Initializing axum server...");
    let port = env::var("PORT").expect("port is not defined as an env var");
    let app_context = AppContext::new(db);
    let router = build_api_router(app_context);

    info!("Routes initialized! Now listening on port {}", port);

    axum::Server::bind(&format!("0.0.0.0:{}", port).parse()?)
        .serve(router.into_make_service())
        .await
        .expect("error while connecting to the server");

    Ok(())
}
