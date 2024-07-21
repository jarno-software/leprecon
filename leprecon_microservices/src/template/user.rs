use askama::Template;
use indexmap::IndexMap;

#[derive(Template)]
#[template(path = "user_information.html", escape = "none")]
pub struct UserInformation<'a> {
    #[template(path = "user_information/account_details.html")]
    pub account_details: AccountDetails,
    pub name_input: NameInput<'a>,
    pub address_input: AddressInput<'a>,
}

#[derive(Template)]
#[template(path = "user_information/account_details.html")]
pub struct AccountDetails {
    pub sub: String,
    pub balance: f64,
    pub currency: String,
}

#[derive(Template)]
#[template(path = "user_information/name_input.html")]
pub struct NameInput<'a> {
    pub inputs: IndexMap<&'a str, Option<String>>,
}

#[derive(Template)]
#[template(path = "user_information/address_input.html")]
pub struct AddressInput<'a> {
    pub inputs: IndexMap<&'a str, Option<String>>,
}
