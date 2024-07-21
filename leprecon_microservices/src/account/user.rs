mod db;
mod model;
mod request;

use self::{
    db::{
        create_customer_details, customer_details_exist, delete_customer_details, delete_user,
        get_customer_details, get_user, insert_user,
    },
    model::{CustomerDetails, User},
    request::delete_user_from_auth_provider,
};

use crate::{
    email::db::delete_email_sessions, user::db::update_customer_details, StateParams, AUTH_HOST,
    CLIENT_ID, CLIENT_SECRET,
};

use askama::Template;
use axum::{extract::State, response::Html, Form};
use indexmap::IndexMap;
use leprecon::{
    auth::{get_valid_jwt, AuthParam},
    template::{self, Snackbar},
    utils::{extract::extract_conn_from_pool, PostgresConn, RedisConn},
};
use reqwest::StatusCode;
use std::collections::HashMap;
use tracing::{debug, error};

pub(super) async fn user_information(
    State(state): State<StateParams>,
    Form(auth_param): Form<AuthParam>,
) -> (StatusCode, Html<String>) {
    let mut snackbar: Snackbar<'_> = Snackbar::default();

    if auth_param.sub.is_empty() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Html(snackbar.render().unwrap()),
        );
    };

    let postgres_conn: PostgresConn = match extract_conn_from_pool(&state.2, &mut snackbar).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    let user: User = match get_user(&auth_param.sub, &postgres_conn).await {
        Ok(v) => v,
        Err(e) => {
            debug!("Could not get user: {:?}", e);
            return (StatusCode::BAD_GATEWAY, Html(snackbar.render().unwrap()));
        }
    };

    let customer_details: CustomerDetails =
        match get_customer_details(&auth_param.sub, &postgres_conn).await {
            Ok(v) => v,
            Err(e) => {
                debug!("Could not get customer details: {:?}", e);
                return (StatusCode::BAD_GATEWAY, Html(snackbar.render().unwrap()));
            }
        };

    let user_template: template::UserInformation = template::UserInformation {
        account_details: template::AccountDetails {
            sub: user.sub,
            balance: user.balance,
            currency: user.currency.to_string(),
        },
        name_input: template::NameInput {
            inputs: IndexMap::from([
                ("first_name", customer_details.first_name),
                ("middle_name", customer_details.middle_name),
                ("last_name", customer_details.last_name),
            ]),
        },
        address_input: template::AddressInput {
            inputs: IndexMap::from([
                ("postal_code", customer_details.postal_code),
                ("street_name", customer_details.street_name),
                ("street_nr", customer_details.street_nr),
                ("premise", customer_details.premise),
                ("settlement", customer_details.settlement),
                ("country", customer_details.country),
                ("country_code", customer_details.country_code),
            ]),
        },
    };

    (StatusCode::OK, Html(user_template.render().unwrap()))
}

pub(super) async fn create_user(
    State(state): State<StateParams>,
    Form(auth_param): Form<AuthParam>,
) -> (StatusCode, Html<String>) {
    let mut snackbar: Snackbar<'_> = Snackbar::default();

    if auth_param.sub.is_empty() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Html(snackbar.render().unwrap()),
        );
    };

    let postgres_conn: PostgresConn = match extract_conn_from_pool(&state.2, &mut snackbar).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    if let Err(e) = insert_user(&auth_param.sub, &postgres_conn).await {
        error!("Could not insert new user: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(snackbar.render().unwrap()),
        );
    }

    snackbar.title = "Succes";
    snackbar.message = "Created user sucessfully";
    snackbar.color = "green";

    (StatusCode::OK, Html(snackbar.render().unwrap()))
}

pub(super) async fn update_user_information(
    State(state): State<StateParams>,
    Form(params): Form<HashMap<String, String>>,
) -> (StatusCode, Html<String>) {
    let mut snackbar: Snackbar<'_> = Snackbar::default();

    let sub: &String = match params.get("sub") {
        Some(v) => v,
        None => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Html(snackbar.render().unwrap()),
            );
        }
    };

    let customer_details: CustomerDetails = CustomerDetails {
        first_name: params.get("first_name").cloned(),
        middle_name: params.get("middle_name").cloned(),
        last_name: params.get("last_name").cloned(),
        postal_code: params.get("postal_code").cloned(),
        street_name: params.get("street_name").cloned(),
        street_nr: params.get("street_nr").cloned(),
        premise: params.get("premise").cloned(),
        settlement: params.get("settlement").cloned(),
        country: params.get("country").cloned(),
        country_code: params.get("country_code").cloned(),
    };

    let postgres_conn: PostgresConn = match extract_conn_from_pool(&state.2, &mut snackbar).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    if customer_details_exist(sub, &postgres_conn).await {
        debug!("Already created customer details entry");
        if let Err(e) = update_customer_details(sub, customer_details, &postgres_conn).await {
            error!("Cannot update customer details entry: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(snackbar.render().unwrap()),
            );
        }
    } else if let Err(e) = create_customer_details(sub, customer_details, &postgres_conn).await {
        error!("Cannot create customer details entry: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(snackbar.render().unwrap()),
        );
    }

    snackbar.title = "Succes";
    snackbar.message = "Updated personal details succesfully";
    snackbar.color = "green";

    (StatusCode::OK, Html(snackbar.render().unwrap()))
}

pub(super) async fn user_balance(
    State(state): State<StateParams>,
    Form(auth_param): Form<AuthParam>,
) -> (StatusCode, Html<String>) {
    let mut snackbar: Snackbar<'_> = Snackbar::default();

    if auth_param.sub.is_empty() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Html(snackbar.render().unwrap()),
        );
    };

    let postgres_conn: PostgresConn = match extract_conn_from_pool(&state.2, &mut snackbar).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    let bal: User = match get_user(&auth_param.sub, &postgres_conn).await {
        Ok(v) => v,
        Err(e) => {
            error!("Could not fetch balance: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(snackbar.render().unwrap()),
            );
        }
    };

    let balance: template::Balance<'_> = template::Balance {
        amount: &bal.balance.to_string(),
        currency: &bal.currency.to_string(),
    };

    (StatusCode::OK, Html(balance.render().unwrap()))
}

pub(super) async fn delete_account(
    State(state): State<StateParams>,
    Form(auth_param): Form<AuthParam>,
) -> (StatusCode, Html<String>) {
    let mut snackbar: Snackbar<'_> = Snackbar::default();

    if auth_param.sub.is_empty() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Html(snackbar.render().unwrap()),
        );
    };

    let postgres_conn: PostgresConn = match extract_conn_from_pool(&state.2, &mut snackbar).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    if let Err(e) = delete_customer_details(&auth_param.sub, &postgres_conn).await {
        error!("Cannot delete customer details entry: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(snackbar.render().unwrap()),
        );
    }

    if let Err(e) = delete_email_sessions(&auth_param.sub, &postgres_conn).await {
        error!("Cannot delete session entrie(s): {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(snackbar.render().unwrap()),
        );
    }

    if let Err(e) = delete_user(&auth_param.sub, &postgres_conn).await {
        error!("Cannot delete user entry: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(snackbar.render().unwrap()),
        );
    }

    let mut lock: tokio::sync::MutexGuard<'_, leprecon::auth::JWT> = state.0.lock().await;
    let req_client: &reqwest::Client = &state.1;

    let redis_conn: RedisConn = match extract_conn_from_pool(&state.3, &mut snackbar).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    *lock = match get_valid_jwt(
        redis_conn,
        req_client,
        AUTH_HOST.get().unwrap(),
        CLIENT_ID.get().unwrap(),
        CLIENT_SECRET.get().unwrap(),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            error!("Could not get valid jwt: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(snackbar.render().unwrap()),
            );
        }
    };

    let res: reqwest::Response = match delete_user_from_auth_provider(
        &auth_param.sub,
        req_client,
        AUTH_HOST.get().unwrap(),
        &lock.access_token,
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            error!("Cannot delete user from auth provider: {:?}", e);

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(snackbar.render().unwrap()),
            );
        }
    };

    if res.status() != reqwest::StatusCode::NO_CONTENT {
        error!("Could not delete user at auth provider");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(snackbar.render().unwrap()),
        );
    }

    snackbar.title = "Succes";
    snackbar.message = "Succesfully deleted account";
    snackbar.color = "green";

    (StatusCode::OK, Html(snackbar.render().unwrap()))
}

#[cfg(test)]
mod test {
    use axum::{body::Body, http::Request};
    use reqwest::{header, Method, StatusCode};
    use tower::ServiceExt;

    use crate::fixture::{assert_body_contains, initialize, seed_database};

    // Get user information
    #[tokio::test]
    async fn test_no_params_provided_get_user_information() {
        let app: axum::Router = initialize().await;

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .uri("/account/user/information")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_body_contains(response, &["Could not process request"]).await;
    }

    #[tokio::test]
    async fn test_no_user() {
        let app: axum::Router = initialize().await;

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .uri("/account/user/information?sub=123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
        assert_body_contains(response, &["Could not process request"]).await;
    }

    #[tokio::test]
    async fn test_no_user_information() {
        let app: axum::Router = initialize().await;
        seed_database().await;

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .uri("/account/user/information?sub=auth0|0002")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_body_contains(response, &["id: auth0|0002", "balance: 0 EUR"]).await;
    }

    // Create user
    #[tokio::test]
    async fn test_no_params_provided_create_user() {
        let app: axum::Router = initialize().await;

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/account/user")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_body_contains(response, &["Could not process request"]).await;
    }

    #[tokio::test]
    async fn test_user_already_exists() {
        let app: axum::Router = initialize().await;
        seed_database().await;

        let params: String = String::from("sub=auth0|0000");

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/account/user")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(params)
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_body_contains(response, &["Could not process request"]).await;
    }

    #[tokio::test]
    async fn test_create_user() {
        let app: axum::Router = initialize().await;
        seed_database().await;

        let params: String = String::from("sub=auth0|0001");

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/account/user")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(params)
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_body_contains(response, &["Created user sucessfully"]).await;
    }

    // Update user
    #[tokio::test]
    async fn test_no_params_provided_update_user_information() {
        let app: axum::Router = initialize().await;

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri("/account/user/information")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_body_contains(response, &["Could not process request"]).await;
    }

    #[tokio::test]
    async fn test_update_user_information_non_existing_user() {
        let app: axum::Router = initialize().await;
        seed_database().await;

        let params: String = String::from("sub=auth0|9999&first_name=Test");

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri("/account/user/information")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(params)
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_body_contains(response, &["Could not process request"]).await;
    }

    #[tokio::test]
    async fn test_update_if_user_information_does_not_exist() {
        let app: axum::Router = initialize().await;
        seed_database().await;

        let first_name: &str = "first";
        let middle_name: &str = "middle";
        let last_name: &str = "last";
        let postal_code: &str = "postal";
        let street_name: &str = "street";
        let street_nr: &str = "nr";
        let premise: &str = "premise";
        let settlement: &str = "settlement";
        let country: &str = "country";
        let country_code: &str = "code";

        let params: String = format!("sub=auth0|0000&first_name={first_name}&middle_name={middle_name}&last_name={last_name}&postal_code={postal_code}&street_name={street_name}&street_nr={street_nr}&premise={premise}&settlement={settlement}&country={country}&country_code={country_code}");

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri("/account/user/information")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(params)
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_body_contains(response, &["Updated personal details succesfully"]).await;
    }

    #[tokio::test]
    async fn test_not_all_user_information_fields() {
        let app: axum::Router = initialize().await;
        seed_database().await;

        let params: String = String::from("sub=auth0|0000&first_name=Test");

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri("/account/user/information")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(params)
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_body_contains(response, &["Updated personal details succesfully"]).await;
    }

    // Get user balance
    #[tokio::test]
    async fn test_no_params_provided_get_user_balance() {
        let app: axum::Router = initialize().await;

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .uri("/account/user/balance")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_body_contains(response, &["Could not process request"]).await;
    }

    #[tokio::test]
    async fn test_user_balance_when_user_does_not_exist() {
        let app: axum::Router = initialize().await;
        seed_database().await;

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .uri("/account/user/balance?sub=123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_body_contains(response, &["Could not process request"]).await;
    }

    #[tokio::test]
    async fn test_get_balance() {
        let app: axum::Router = initialize().await;
        seed_database().await;

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .uri("/account/user/balance?sub=auth0|0000")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_body_contains(response, &["0", "EUR"]).await;
    }

    // Delete user
    #[tokio::test]
    async fn test_no_params_provided_delete_user() {
        let app: axum::Router = initialize().await;

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri("/account/user")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_body_contains(response, &["Could not process request"]).await;
    }

    #[tokio::test]
    async fn test_delete_non_existing_user() {
        let app: axum::Router = initialize().await;
        seed_database().await;

        let params: String = String::from("sub=123");

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri("/account/user")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(params)
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_body_contains(response, &["Could not process request"]).await;
    }

    #[tokio::test]
    async fn test_delete_user_no_user_information() {
        let app: axum::Router = initialize().await;
        seed_database().await;

        let params: String = String::from("sub=auth0|0004");

        let response: axum::http::Response<Body> = app
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri("/account/user")
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .body(params)
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_body_contains(response, &["Succesfully deleted account"]).await;
    }
}
