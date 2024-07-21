use crate::utils::RedisConn;

use super::JWT;

use chrono::Local;
use redis::AsyncCommands;
use std::error::Error;
use tracing::debug;

pub(crate) async fn get_jwt_from_valkey(valkey_conn: &mut RedisConn<'_>) -> Option<JWT> {
    match valkey_conn.hget("session:account", "jwt").await {
        Ok(v) => {
            let value: String = v;
            match serde_json::from_str::<JWT>(&value) {
                Ok(v) => {
                    if v.expires_in > Local::now() {
                        debug!("Fetched jwt from session");
                        return Some(v);
                    }
                }
                Err(e) => debug!("Could not deserialize jwt: {:?}", e),
            };
        }
        Err(e) => debug!("Could not get jwt from session store: {:?}", e),
    };

    None
}

pub(crate) async fn store_jwt(mut conn: RedisConn<'_>, token: &JWT) -> Result<(), Box<dyn Error>> {
    let v: String = serde_json::to_string(token)?;

    conn.hset("session:account", "jwt", v).await?;
    conn.expire_at("session:account", token.expires_in.timestamp())
        .await?;

    Ok(())
}
