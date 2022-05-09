use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub mod user_repository;

pub async fn initialize_db(connection_string: &str) -> Pool<Postgres> {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await
        .expect("could not connect to the database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("could not run database migrations");

    db
}
