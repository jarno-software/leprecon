use reqwest::{Client, Response};

pub(super) async fn delete_user_from_auth_provider(
    sub: &str,
    req_client: &Client,
    auth_host: &str,
    access_token: &str,
) -> Result<Response, reqwest::Error> {
    req_client
        .delete(format!("{auth_host}/api/v2/users/{sub}"))
        .bearer_auth(access_token)
        .send()
        .await
}
