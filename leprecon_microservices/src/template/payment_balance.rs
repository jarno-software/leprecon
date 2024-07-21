use askama::Template;

#[derive(Template)]
#[template(path = "payment_balance.html")]
pub struct PaymentBalance;
