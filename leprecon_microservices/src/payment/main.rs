mod balance;

use axum::{serve, Router};
use balance::{add_balance, get_balance_page};
use leprecon::{broker::init_broker, signals::shutdown_signal, utils::configure_tracing};
use rabbitmq_stream_client::types::ByteCapacity;
use std::{env, error::Error, sync::OnceLock};
use tokio::net::TcpListener;
use tracing::{error, info};

// Host variables
static HOST: OnceLock<String> = OnceLock::new();
static LOG_LEVEL: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize env variables
    init_env();

    // Initialize broker environment
    let environment = init_broker().await;
    let stream = "balance_update";
    let create_response = environment
        .stream_creator()
        .max_length(ByteCapacity::GB(5))
        .create(stream)
        .await;

    if let Err(e) = create_response {
        error!("Error creating stream: {:?} {:?}", stream, e);
    }

    let producer: rabbitmq_stream_client::Producer<rabbitmq_stream_client::NoDedup> =
        environment.producer().build(stream).await?;

    // Configure logging
    configure_tracing(LOG_LEVEL.get().unwrap());

    // Build application and listen to incoming requests.
    let app: Router = build_app(producer);
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
    HOST.get_or_init(|| env::var("PAYMENT_HOST").unwrap());
    LOG_LEVEL.get_or_init(|| env::var("LOG_LEVEL").unwrap());
}

/// Builds the application.
fn build_app(
    producer: rabbitmq_stream_client::Producer<rabbitmq_stream_client::NoDedup>,
) -> Router {
    Router::new().route(
        "/payment/balance",
        axum::routing::post(add_balance)
            .get(get_balance_page)
            .with_state(producer),
    )
}
