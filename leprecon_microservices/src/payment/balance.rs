mod model;

use askama::Template;
use axum::{extract::State, response::Html, Form};
use leprecon::template::{self, Snackbar};
use rabbitmq_stream_client::types::Message;
use reqwest::StatusCode;
use tracing::error;

pub(super) async fn get_balance_page() -> (StatusCode, Html<String>) {
    let templ: template::PaymentBalance = template::PaymentBalance;
    (StatusCode::OK, Html(templ.render().unwrap()))
}

pub(super) async fn add_balance(
    state: State<rabbitmq_stream_client::Producer<rabbitmq_stream_client::NoDedup>>,
    Form(balance): Form<model::Balance>,
) -> (StatusCode, Html<String>) {
    let mut snackbar: Snackbar<'_> = Snackbar::default();

    if let Err(e) = state
        .0
        .send_with_confirm(
            Message::builder()
                .body(format!(
                    "sub: {:?}, amount: {:?}",
                    balance.sub.to_string(),
                    balance.amount.to_string()
                ))
                .build(),
        )
        .await
    {
        error!("Error while publishing message: {:?}", e);

        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(snackbar.render().unwrap()),
        );
    };

    snackbar.title = "Succes";
    snackbar.message = "Succesfully added balance";
    snackbar.color = "green";

    (StatusCode::OK, Html(snackbar.render().unwrap()))
}
