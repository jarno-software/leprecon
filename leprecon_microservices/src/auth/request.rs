use axum::http::HeaderValue;
use reqwest::Response;
use std::collections::HashMap;

pub(crate) async fn jwt_from_auth_provider(
    req_client: &reqwest::Client,
    auth_host: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<Response, reqwest::Error> {
    // Create headers
    let mut headers = reqwest::header::HeaderMap::new();
    let content_type: HeaderValue =
        HeaderValue::from_str("application/x-www-form-urlencoded").unwrap();
    headers.insert("Content-Type", content_type);

    // Config
    let token_url: String = format!("{auth_host}/oauth/token");
    let audience: String = format!("{auth_host}/api/v2/");
    let params: HashMap<&str, &str> = HashMap::from([
        ("grant_type", "client_credentials"),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("audience", &audience),
    ]);

    // Request
    req_client.post(token_url).form(&params).send().await
}
