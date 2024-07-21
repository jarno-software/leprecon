mod email;
mod embedded;
mod fixture;
mod model;
mod user;

use axum::{serve, Router};
use bb8_postgres::{bb8::Pool, PostgresConnectionManager};
use bb8_redis::RedisConnectionManager;
use email::email_verification;
use fixture::{add_currency, add_users, create_account_db};
use futures::StreamExt;
use leprecon::{
    auth::{get_valid_jwt, JWT},
    broker::init_broker,
    signals::shutdown_signal,
    utils::{configure_tracing, create_conn_pool},
};
use rabbitmq_stream_client::types::{ByteCapacity, OffsetSpecification};
use std::{
    env,
    error::Error,
    ops::DerefMut,
    sync::{Arc, OnceLock},
    time::Duration,
};
use tokio::{net::TcpListener, sync::Mutex, task};
use tokio_postgres::NoTls;
use tracing::{error, info};
use user::{create_user, delete_account, update_user_information, user_balance, user_information};

type StateParams = (
    Arc<tokio::sync::Mutex<JWT>>,
    reqwest::Client,
    bb8_postgres::bb8::Pool<PostgresConnectionManager<NoTls>>,
    bb8_postgres::bb8::Pool<RedisConnectionManager>,
);

// Host variables
static HOST: OnceLock<String> = OnceLock::new();
static LOG_LEVEL: OnceLock<String> = OnceLock::new();

// DB variables
static ACCOUNT_CONN: OnceLock<String> = OnceLock::new();

// Auth variables
static AUTH_HOST: OnceLock<String> = OnceLock::new();
static CLIENT_ID: OnceLock<String> = OnceLock::new();
static CLIENT_SECRET: OnceLock<String> = OnceLock::new();

// VALKEY variables
static VALKEY_CONN: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize env variables
    init_env();

    // Configure logging
    configure_tracing(LOG_LEVEL.get().unwrap());

    // Http client (holds connection pool internally)
    let req_client: reqwest::Client = reqwest::Client::new();

    // Initialize broker connection
    let environment = init_broker().await;
    let stream = "balance_update";
    let create_response = environment
        .stream_creator()
        .max_length(ByteCapacity::GB(5))
        .create(stream)
        .await;

    if let Err(e) = create_response {
        println!("Error creating stream: {:?} {:?}", stream, e);
    }

    let mut consumer = environment
        .consumer()
        .offset(OffsetSpecification::First)
        .build(stream)
        .await
        .unwrap();

    consumer.handle();
    task::spawn(async move {
        while let Some(delivery) = consumer.next().await {
            let d = delivery.unwrap();
            info!(
                "Got message: {:#?} with offset: {}",
                d.message()
                    .data()
                    .map(|data| String::from_utf8(data.to_vec()).unwrap()),
                d.offset(),
            );
        }
    });

    // Create account db if not exists
    create_account_db().await;

    // Connection pool config
    let connection_timeout: Duration = Duration::from_secs(10);
    let max_size: u32 = 20;

    // Postgres connection pool
    let postgres_manager: PostgresConnectionManager<tokio_postgres::NoTls> =
        PostgresConnectionManager::new_from_stringlike(
            ACCOUNT_CONN.get().unwrap(),
            tokio_postgres::NoTls,
        )?;
    let postgres_pool: Pool<PostgresConnectionManager<NoTls>> =
        create_conn_pool(postgres_manager, connection_timeout, max_size).await?;

    // Create database if not exist
    let (db_client, connection) = tokio_postgres::connect(&env::var("DB_CONN").unwrap(), NoTls)
        .await
        .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    if let Err(e) = db_client.query("CREATE DATABASE account", &[]).await {
        error!("Database already exists: {:?}", e);
    };

    // Run migrations
    embedded::migrations::runner()
        .run_async(postgres_pool.get().await?.deref_mut())
        .await?;

    add_currency(postgres_pool.get().await?.deref_mut()).await;
    let sub = env::var("SUB_NOT_VERIFIED").unwrap();
    add_users(postgres_pool.get().await?.deref_mut(), &vec![&sub]).await;

    // Redis connection pool
    let redis_manager: RedisConnectionManager =
        RedisConnectionManager::new(VALKEY_CONN.get().unwrap().to_owned()).unwrap();
    let redis_pool: Pool<RedisConnectionManager> =
        create_conn_pool(redis_manager, connection_timeout, max_size).await?;

    // Get valid access token
    let jwt: JWT = get_valid_jwt(
        redis_pool.get().await?,
        &req_client,
        AUTH_HOST.get().unwrap(),
        CLIENT_ID.get().unwrap(),
        CLIENT_SECRET.get().unwrap(),
    )
    .await?;

    // Build application and listen to incoming requests.
    let app: Router = build_app(
        Arc::new(Mutex::new(jwt)),
        req_client,
        postgres_pool,
        redis_pool,
    );
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
    HOST.get_or_init(|| env::var("ACCOUNT_HOST").unwrap());
    LOG_LEVEL.get_or_init(|| env::var("LOG_LEVEL").unwrap());

    ACCOUNT_CONN.get_or_init(|| env::var("ACCOUNT_CONN").unwrap());

    AUTH_HOST.get_or_init(|| env::var("AUTH_HOST").unwrap());
    CLIENT_ID.get_or_init(|| env::var("CLIENT_ID_ACCOUNT").unwrap());
    CLIENT_SECRET.get_or_init(|| env::var("CLIENT_SECRET_ACCOUNT").unwrap());

    VALKEY_CONN.get_or_init(|| env::var("VALKEY_CONN").unwrap());
}

/// Builds the application.
fn build_app(
    jwt: Arc<Mutex<JWT>>,
    req_client: reqwest::Client,
    postgres_pool: Pool<PostgresConnectionManager<NoTls>>,
    redis_pool: Pool<RedisConnectionManager>,
) -> Router {
    Router::new()
        .route(
            "/account/email/verification",
            axum::routing::post(email_verification),
        )
        .route("/account/user/balance", axum::routing::get(user_balance))
        .route(
            "/account/user/information",
            axum::routing::get(user_information).put(update_user_information),
        )
        .route(
            "/account/user",
            axum::routing::post(create_user).delete(delete_account),
        )
        .with_state((jwt, req_client, postgres_pool, redis_pool))
}
