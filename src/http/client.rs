use super::{HttpResponse, HttpResult};
use anyhow::Context;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RequestData {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

pub async fn send(req: RequestData) -> HttpResult {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(crate::config::DEFAULT_TIMEOUT_SECS))
        .user_agent("flame/0.1.0")
        .build()
        .context("Failed to build HTTP client")?;

    let method = reqwest::Method::from_bytes(req.method.as_bytes())
        .context("Invalid HTTP method")?;

    let mut builder = client.request(method, &req.url);

    for (k, v) in &req.headers {
        builder = builder.header(k, v);
    }

    if let Some(body) = req.body {
        builder = builder.body(body);
    }

    let start = Instant::now();
    let resp = builder.send().await.context("Request failed")?;
    let duration_ms = start.elapsed().as_millis();

    let status = resp.status();
    let status_text = status.canonical_reason().unwrap_or("").to_string();

    let headers: Vec<(String, String)> = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("<binary>").to_string()))
        .collect();

    let body = resp.text().await.unwrap_or_default();

    Ok(HttpResponse {
        status: status.as_u16(),
        status_text,
        headers,
        body,
        duration_ms,
    })
}