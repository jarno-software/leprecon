pub mod extract;

use bb8_postgres::PostgresConnectionManager;
use bb8_redis::{
    bb8::{ManageConnection, Pool},
    RedisConnectionManager,
};
use std::{str::FromStr, time::Duration};
use tokio_postgres::NoTls;
use tracing::Level;
use tracing_subscriber::{
    fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt,
};

pub type RedisConn<'a> = bb8_redis::bb8::PooledConnection<'a, RedisConnectionManager>;
pub type PostgresConn<'a> = bb8_redis::bb8::PooledConnection<'a, PostgresConnectionManager<NoTls>>;

/// Configure tracing with tracing_subscriber.
pub fn configure_tracing(log_level: &str) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout.with_max_level(Level::from_str(log_level).unwrap())),
        )
        .init();
}

pub async fn create_conn_pool<M>(
    manager: M,
    connection_timeout: Duration,
    max_size: u32,
) -> Result<Pool<M>, M::Error>
where
    M: ManageConnection,
{
    Pool::builder()
        .connection_timeout(connection_timeout)
        .max_size(max_size)
        .build(manager)
        .await
}
