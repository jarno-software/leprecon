use super::model::{Currency, CustomerDetails, User};

use leprecon::utils::PostgresConn;
use std::{error::Error, str::FromStr};
use tokio_postgres::Row;
use tracing::debug;

pub(super) async fn insert_user(
    sub: &str,
    db_client: &PostgresConn<'_>,
) -> Result<Vec<Row>, tokio_postgres::Error> {
    db_client
        .query(
            "INSERT INTO users(sub, balance, currency_id) VALUES($1, 0.00, 1)",
            &[&sub],
        )
        .await
}

pub(super) async fn get_user(sub: &str, conn: &PostgresConn<'_>) -> Result<User, Box<dyn Error>> {
    let r: Row = conn
        .query_one("SELECT * FROM users INNER JOIN currencies ON currencies.id = users.currency_id WHERE sub=$1 LIMIT 1", &[&sub])
        .await?;

    Ok(User {
        sub: r.get("sub"),
        balance: r.get("balance"),
        currency: Currency::from_str(r.get("acronym"))?,
    })
}

pub(super) async fn delete_user(
    sub: &str,
    db_client: &PostgresConn<'_>,
) -> Result<Vec<Row>, tokio_postgres::Error> {
    db_client
        .query("DELETE FROM users WHERE sub = $1", &[&sub])
        .await
}

pub(super) async fn customer_details_exist(sub: &str, db_client: &PostgresConn<'_>) -> bool {
    match db_client
        .query_one(
            "SELECT * FROM customer_details LEFT JOIN users ON users.id = customer_details.user_id WHERE sub=$1 LIMIT 1",
            &[&sub],
        )
        .await
    {
        Err(e) => {
            debug!("No customer details in db: {:?}", e);
            false
        }
        _ => true,
    }
}

pub(super) async fn get_customer_details(
    sub: &str,
    db_client: &PostgresConn<'_>,
) -> Result<CustomerDetails, Box<dyn Error>> {
    let r: Row = db_client
        .query_one(
            "SELECT * FROM customer_details RIGHT JOIN users ON users.id = customer_details.user_id WHERE users.sub=$1 LIMIT 1",
            &[&sub],
        )
        .await?;

    Ok(CustomerDetails {
        first_name: r.get("first_name"),
        middle_name: r.get("middle_name"),
        last_name: r.get("last_name"),
        postal_code: r.get("postal_code"),
        street_name: r.get("street_name"),
        street_nr: r.get("street_nr"),
        premise: r.get("premise"),
        settlement: r.get("settlement"),
        country: r.get("country"),
        country_code: r.get("country_code"),
    })
}

pub(super) async fn create_customer_details(
    sub: &str,
    customer_details: CustomerDetails,
    db_client: &PostgresConn<'_>,
) -> Result<Vec<Row>, tokio_postgres::Error> {
    db_client
        .query(
            "WITH userId AS (SELECT id FROM users WHERE sub = $1) INSERT INTO customer_details(first_name, middle_name, last_name, postal_code, street_name, street_nr, premise, settlement, country, country_code, user_id) VALUES($2, $3, $4, $5, $6, $7, $8, $9, $10, $11, (SELECT id FROM userId))",
            &[&sub, &customer_details.first_name, &customer_details.middle_name, &customer_details.last_name, &customer_details.postal_code, &customer_details.street_name, &customer_details.street_nr, &customer_details.premise, &customer_details.settlement, &customer_details.country, &customer_details.country_code],
        )
        .await
}

pub(super) async fn update_customer_details(
    sub: &str,
    customer_details: CustomerDetails,
    db_client: &PostgresConn<'_>,
) -> Result<Vec<Row>, tokio_postgres::Error> {
    db_client
        .query(
            "WITH userId AS (SELECT id FROM users WHERE sub = $1) UPDATE customer_details SET first_name = $2, middle_name = $3, last_name = $4, postal_code = $5, street_name = $6, street_nr = $7, premise = $8, settlement = $9, country = $10, country_code = $11 WHERE user_id = (SELECT id FROM userId)",
            &[&sub, &customer_details.first_name, &customer_details.middle_name, &customer_details.last_name, &customer_details.postal_code, &customer_details.street_name, &customer_details.street_nr, &customer_details.premise, &customer_details.settlement, &customer_details.country, &customer_details.country_code],
        )
        .await
}

pub(super) async fn delete_customer_details(
    sub: &str,
    db_client: &PostgresConn<'_>,
) -> Result<Option<Row>, tokio_postgres::Error> {
    db_client.query_opt(
        "WITH userId AS (SELECT id FROM users WHERE sub = $1) DELETE FROM customer_details WHERE user_id = (SELECT id FROM userId)",
        &[&sub],
    )
    .await
}
