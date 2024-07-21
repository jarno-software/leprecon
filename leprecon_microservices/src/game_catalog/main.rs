mod catalog;
mod fixture;

use axum::{serve, Router};
use catalog::get_catalog;
use fixture::seed_db;
use leprecon::{signals::shutdown_signal, utils::configure_tracing};
use mongodb::options::ClientOptions;
use std::{env, error::Error, sync::OnceLock};
use tokio::net::TcpListener;
use tracing::info;

// Host variables
static HOST: OnceLock<String> = OnceLock::new();
static LOG_LEVEL: OnceLock<String> = OnceLock::new();

// Mongo
static GAME_CATALOG_CONN: OnceLock<String> = OnceLock::new();
static GAME_CATALOG_DB: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize env variables
    init_env();

    // Configure logging
    configure_tracing(LOG_LEVEL.get().unwrap());

    // Mongo
    let client_options: ClientOptions =
        ClientOptions::parse(GAME_CATALOG_CONN.get().unwrap()).await?;
    let mongo_client: mongodb::Client = mongodb::Client::with_options(client_options).unwrap();
    let mongo_db: mongodb::Database = mongo_client.database(GAME_CATALOG_DB.get().unwrap());

    // Seed database
    seed_db(&mongo_db).await;

    // Build application and listen to incoming requests.
    let app: Router = build_app(mongo_db);
    let listener: TcpListener = TcpListener::bind(HOST.get().unwrap()).await?;

    info!("Running application");

    // Run the app.
    serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// Initialize env variables
fn init_env() {
    HOST.get_or_init(|| env::var("GAME_CATALOG_HOST").unwrap());
    LOG_LEVEL.get_or_init(|| env::var("LOG_LEVEL").unwrap());

    GAME_CATALOG_CONN.get_or_init(|| env::var("GAME_CATALOG_CONN").unwrap());
    GAME_CATALOG_DB.get_or_init(|| env::var("GAME_CATALOG_DB").unwrap());
}

/// Builds the application.
fn build_app(mongo_db: mongodb::Database) -> Router {
    Router::new().route(
        "/game/catalog",
        axum::routing::get(get_catalog).with_state(mongo_db),
    )
}
