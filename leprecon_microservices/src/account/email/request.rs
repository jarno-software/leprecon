use std::collections::HashMap;

use axum::http::HeaderValue;
use reqwest::Response;

pub(super) async fn send_email_verification(
    req_client: &reqwest::Client,
    sub: &str,
    client_id: &str,
    auth_host: &str,
    access_token: &str,
) -> Result<Response, reqwest::Error> {
    // Set headers
    let mut headers = reqwest::header::HeaderMap::new();
    let content_type: HeaderValue = HeaderValue::from_str("application/json").unwrap();
    headers.insert("Content-Type", content_type.clone());
    headers.insert("Accept", content_type);

    // Setup
    let map: HashMap<&str, &str> = HashMap::from([("user_id", sub), ("client_id", client_id)]);

    // Send request
    req_client
        .post(format!("{auth_host}/api/v2/jobs/verification-email"))
        .json(&map)
        .bearer_auth(access_token)
        .send()
        .await
}
