use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct EmailParams {
    #[serde(default)]
    pub sub: String,

    #[serde(default)]
    pub email_verified: String,
}
