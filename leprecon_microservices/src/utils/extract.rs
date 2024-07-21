use crate::template::Snackbar;

use askama::Template;
use axum::response::Html;
use bb8_redis::bb8::{ManageConnection, Pool, PooledConnection};
use reqwest::StatusCode;
use tracing::debug;

pub async fn extract_conn_from_pool<'a, M>(
    pool: &'a Pool<M>,
    snackbar: &mut Snackbar<'_>,
) -> Result<PooledConnection<'a, M>, (StatusCode, Html<String>)>
where
    M: ManageConnection,
{
    match pool.get().await {
        Ok(v) => Ok(v),
        Err(e) => {
            debug!("Cannot get connection from pool: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(snackbar.render().unwrap()),
            ))
        }
    }
}
