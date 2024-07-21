use askama::Template;

#[derive(Template)]
#[template(path = "balance.html")]
pub struct Balance<'a> {
    pub amount: &'a str,
    pub currency: &'a str,
}
