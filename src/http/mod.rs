pub mod client;

pub use client::{send, RequestData};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub duration_ms: u128,
}

pub type HttpResult = Result<HttpResponse>;