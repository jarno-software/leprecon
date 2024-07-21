mod db;
mod model;
mod request;

pub use model::*;

use self::{db::store_jwt, request::jwt_from_auth_provider};

use crate::{auth::db::get_jwt_from_valkey, utils::RedisConn};

use reqwest::StatusCode;
use std::error::Error;
use tracing::{debug, error};

pub async fn get_valid_jwt(
    mut valkey_conn: RedisConn<'_>,
    req_client: &reqwest::Client,
    auth_host: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<JWT, Box<dyn Error>> {
    // Get valid jwt from valkey
    if let Some(v) = get_jwt_from_valkey(&mut valkey_conn).await {
        return Ok(v);
    }

    // Get new token from provider
    let response: reqwest::Response =
        jwt_from_auth_provider(req_client, auth_host, client_id, client_secret).await?;
    let status = response.status();
    let text = response.text().await.unwrap();
    if status != StatusCode::OK {
        error!("JWT fetch body:\n{:?}", text);
        Err("StatusCode not OK")?
    }

    let jwt: JWT = serde_json::from_str(&text)?;

    // Store jwt in valkey
    store_jwt(valkey_conn, &jwt).await?;

    debug!("Fetched jwt from auth provider");

    Ok(jwt)
}
