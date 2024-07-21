use chrono::{Duration, Local};
use leprecon::utils::PostgresConn;
use tokio_postgres::Row;
use tracing::debug;

use crate::model::SessionType;

pub(super) async fn verification_already_send(db_client: &PostgresConn<'_>, sub: &str) -> bool {
    match db_client
        .query_one(
            "SELECT * FROM sessions INNER JOIN users ON users.id = sessions.user_id WHERE expires > now() AND sub=$1 AND type=$2 ORDER BY expires DESC LIMIT 1",
            &[&sub, &SessionType::Verification.to_string()],
        )
        .await
    {
        Err(e) => {
            debug!("No email verification session in db: {:?}", e);
            false
        }
        _ => true,
    }
}

pub(super) async fn create_verification_session(
    db_client: &PostgresConn<'_>,
    sub: &str,
) -> Result<Vec<Row>, tokio_postgres::Error> {
    let expires = 3600; // 60 minutes

    db_client
        .query(
            "WITH userId AS (SELECT id FROM users WHERE sub = $1) INSERT INTO sessions(expires, type, user_id) VALUES($2, $3, (SELECT id FROM userId))",
            &[&sub, &(Local::now() + Duration::seconds(expires)), &SessionType::Verification.to_string()],
        )
        .await
}

pub(crate) async fn delete_email_sessions(
    sub: &str,
    db_client: &PostgresConn<'_>,
) -> Result<Option<Row>, tokio_postgres::Error> {
    db_client.query_opt(
        "WITH userId AS (SELECT id FROM users WHERE sub = $1) DELETE FROM sessions WHERE user_id = (SELECT id FROM userId)",
        &[&sub],
    )
    .await
}
