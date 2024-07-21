use std::{env, sync::Arc};

use axum::{
    body::{self, Body},
    Router,
};
use bb8_postgres::PostgresConnectionManager;
use bb8_redis::{bb8::Pool, RedisConnectionManager};
use chrono::{DateTime, Local};
use leprecon::{
    auth::{get_valid_jwt, JWT},
    utils::create_conn_pool,
};
use tokio::sync::Mutex;
use tokio_postgres::NoTls;
use tracing::error;

use crate::{
    build_app, embedded, init_env, ACCOUNT_CONN, AUTH_HOST, CLIENT_ID, CLIENT_SECRET, VALKEY_CONN,
};

#[allow(dead_code)]
pub(crate) async fn initialize() -> Router {
    init_env();

    let connection_timeout: std::time::Duration = std::time::Duration::from_secs(10);
    let max_size: u32 = 20;

    let postgres_manager: PostgresConnectionManager<tokio_postgres::NoTls> =
        PostgresConnectionManager::new_from_stringlike(
            ACCOUNT_CONN.get().unwrap(),
            tokio_postgres::NoTls,
        )
        .unwrap();
    let postgres_pool: Pool<PostgresConnectionManager<NoTls>> =
        create_conn_pool(postgres_manager, connection_timeout, max_size)
            .await
            .unwrap();

    let redis_manager: RedisConnectionManager =
        RedisConnectionManager::new(VALKEY_CONN.get().unwrap().to_owned()).unwrap();
    let redis_pool: Pool<RedisConnectionManager> =
        create_conn_pool(redis_manager, connection_timeout, max_size)
            .await
            .unwrap();

    let req_client: reqwest::Client = reqwest::Client::new();

    let jwt: JWT = get_valid_jwt(
        redis_pool.get().await.unwrap(),
        &req_client,
        AUTH_HOST.get().unwrap(),
        CLIENT_ID.get().unwrap(),
        CLIENT_SECRET.get().unwrap(),
    )
    .await
    .unwrap();

    build_app(
        Arc::new(Mutex::new(jwt)),
        req_client,
        postgres_pool,
        redis_pool,
    )
}

#[allow(dead_code)]
static INITIALISED: Mutex<bool> = Mutex::const_new(false);

#[allow(dead_code)]
pub(crate) async fn seed_database() {
    let mut initialised = INITIALISED.lock().await;
    if *initialised {
        return;
    }

    create_account_db().await;

    let (mut db_client, connection) = tokio_postgres::connect(ACCOUNT_CONN.get().unwrap(), NoTls)
        .await
        .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    // Create tables
    embedded::migrations::runner()
        .run_async(&mut db_client)
        .await
        .unwrap();

    let sub: String = env::var("SUB_NOT_VERIFIED").unwrap();
    let subs: Vec<&str> = vec!["auth0|0000", "auth0|0002", "auth0|0003", "auth0|0004", &sub];

    add_currency(&db_client).await;
    add_users(&db_client, &subs).await;
    add_email_session(&db_client, subs[0]).await;

    *initialised = true;
}

pub async fn create_account_db() {
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
}

// !TODO Remove once the currency table has moved
pub async fn add_currency(conn: &tokio_postgres::Client) {
    conn.query(
        "INSERT INTO currencies(acronym) VALUES('EUR') ON CONFLICT DO NOTHING",
        &[],
    )
    .await
    .unwrap();
}

pub async fn add_users(conn: &tokio_postgres::Client, subs: &Vec<&str>) {
    for sub in subs {
        conn.query(
            "INSERT INTO users(sub, balance, currency_id) VALUES($1, 0, 1) ON CONFLICT DO NOTHING",
            &[&sub],
        )
        .await
        .unwrap();
    }
}

async fn add_email_session(conn: &tokio_postgres::Client, sub: &str) {
    let expires: DateTime<Local> = Local::now() + chrono::Duration::seconds(3600);

    conn.query(
        "WITH userId AS (SELECT id FROM users WHERE sub = $1) INSERT INTO sessions(expires, type, user_id) VALUES($2, 'Verification', (SELECT id FROM userId))",
        &[&sub, &expires],
    )
    .await
    .unwrap();
}

#[allow(dead_code)]
pub(crate) async fn assert_body_contains(response: axum::http::Response<Body>, body: &[&str]) {
    let bytes: body::Bytes = body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str: String = String::from_utf8(bytes.to_vec()).unwrap();

    for s in body {
        assert!(body_str.contains(s));
    }
}
