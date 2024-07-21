use std::{
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub(super) enum Currency {
    EUR,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub(super) struct User {
    pub sub: String,
    pub balance: f64,
    pub currency: Currency,
}

pub(super) struct CustomerDetails {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub postal_code: Option<String>,
    pub street_name: Option<String>,
    pub street_nr: Option<String>,
    pub premise: Option<String>,
    pub settlement: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug)]
pub(super) struct ParseCurrencyError;

impl Error for ParseCurrencyError {}

impl Display for ParseCurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseCurrencyError")
    }
}

impl FromStr for Currency {
    type Err = ParseCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EUR" => Ok(Currency::EUR),
            _ => Ok(Currency::EUR),
        }
    }
}
