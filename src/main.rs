use std::env;

use sqlx::postgres::PgPoolOptions;
use tracing::{info, span, Level};
use tracing_subscriber::FmtSubscriber;

use crate::errors::ConduitError;
use crate::lib::routes::build_api_router;

extern crate argon2;

mod errors;
mod lib;

#[tokio::main]
async fn main() -> Result<(), ConduitError> {
    // Build out the logging instance using tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let _init_span = span!(Level::INFO, "initialize_api").entered();

    info!("Loading environment variables...");
    dotenv::dotenv().ok();

    let connection_string =
        env::var("CONNECTION_STRING").expect("connection string is not available");

    info!("Environment loaded! Configuration Postgres connection...");
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
        .expect("could not connect to the database");

    info!("Postgres connection established, running migrations...");
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("could not run database migrations");

    info!("Migrations successfully ran! Initializing axum server...");
    let port = env::var("PORT").expect("port is not defined as an env var");
    let app = build_api_router();

    info!("Routes initialized! Now listening on port {}", port);
    _init_span.exit();
    axum::Server::bind(&format!("0.0.0.0:{}", port).parse()?)
        .serve(app.into_make_service())
        .await
        .expect("error while connecting to the server");

    Ok(())
}
