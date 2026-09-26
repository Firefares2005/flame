use crate::http::{self, HttpResult, RequestData};
use crate::storage::{self, SavedRequest};
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Request,
    Response,
    Collections,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
        }
    }

    pub fn next(&self) -> Method {
        match self {
            Method::GET => Method::POST,
            Method::POST => Method::PUT,
            Method::PUT => Method::DELETE,
            Method::DELETE => Method::PATCH,
            Method::PATCH => Method::HEAD,
            Method::HEAD => Method::OPTIONS,
            Method::OPTIONS => Method::GET,
        }
    }

    pub fn prev(&self) -> Method {
        match self {
            Method::GET => Method::OPTIONS,
            Method::POST => Method::GET,
            Method::PUT => Method::POST,
            Method::DELETE => Method::PUT,
            Method::PATCH => Method::DELETE,
            Method::HEAD => Method::PATCH,
            Method::OPTIONS => Method::HEAD,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub method: String,
    pub url: String,
    pub status: u16,
    pub duration_ms: u128,
}

#[derive(Debug, Clone)]
pub struct ResponseState {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub duration_ms: u128,
    pub pretty_body: String,
}

pub struct App {
    pub should_quit: bool,
    pub active_tab: Tab,
    pub method: Method,
    pub url: String,
    pub headers_input: String,
    pub body_input: String,
    pub focus: usize,

    pub response: Option<ResponseState>,
    pub error: Option<String>,
    pub loading: bool,

    pub collections: Vec<SavedRequest>,
    pub collection_path: String,
    pub history: Vec<HistoryEntry>,

    pub variables: HashMap<String, String>,
    pub sidebar_selected: usize,

    pub rx: Option<mpsc::Receiver<HttpResult>>,

    // New: scrolling for response body
    pub response_scroll: u16,
    // New: notification message
    pub status_message: Option<String>,
}

impl App {
    pub fn new(collection_path: String) -> Result<Self> {
        let collections = storage::load_collections(&collection_path).unwrap_or_default();
        let mut variables = HashMap::new();
        variables.insert("base_url".to_string(), "https://httpbin.org".to_string());

        Ok(Self {
            should_quit: false,
            active_tab: Tab::Request,
            method: Method::GET,
            url: "https://httpbin.org/get".to_string(),
            headers_input: "Accept: application/json".to_string(),
            body_input: String::new(),
            focus: 0,
            response: None,
            error: None,
            loading: false,
            collections,
            collection_path,
            history: Vec::new(),
            variables,
            sidebar_selected: 0,
            rx: None,
            response_scroll: 0,
            status_message: None,
        })
    }

    pub fn focus_next(&mut self) {
        self.focus = (self.focus + 1) % 3;
    }

    pub fn focus_prev(&mut self) {
        if self.focus == 0 {
            self.focus = 2;
        } else {
            self.focus -= 1;
        }
    }

    pub fn current_input_mut(&mut self) -> &mut String {
        match self.focus {
            0 => &mut self.url,
            1 => &mut self.headers_input,
            _ => &mut self.body_input,
        }
    }

    pub fn substitute_vars(&self, input: &str) -> String {
        let mut out = input.to_string();
        for (k, v) in &self.variables {
            out = out.replace(&format!("{{{{{}}}}}", k), v);
        }
        out
    }

    pub fn send_request(&mut self) {
        if self.loading {
            return;
        }

        let url = self.substitute_vars(&self.url);
        if url.trim().is_empty() {
            self.error = Some("URL is empty".to_string());
            return;
        }

        let headers: Vec<(String, String)> = self
            .headers_input
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, ':');
                let k = parts.next()?.trim().to_string();
                let v = parts.next()?.trim().to_string();
                if k.is_empty() { None } else { Some((k, v)) }
            })
            .collect();

        let body = self.body_input.clone();
        let method = self.method;

        let req = RequestData {
            method: method.as_str().to_string(),
            url,
            headers,
            body: if body.trim().is_empty() { None } else { Some(body) },
        };

        let (tx, rx) = mpsc::channel(1);
        self.rx = Some(rx);
        self.loading = true;
        self.error = None;
        self.response_scroll = 0;

        tokio::spawn(async move {
            let result = http::send(req).await;
            let _ = tx.send(result).await;
        });
    }

    pub fn poll_response(&mut self) {
        if let Some(rx) = &mut self.rx {
            if let Ok(result) = rx.try_recv() {
                match result {
                    Ok(res) => {
                        let pretty = match serde_json::from_str::<serde_json::Value>(&res.body) {
                            Ok(v) => serde_json::to_string_pretty(&v).unwrap_or(res.body.clone()),
                            Err(_) => res.body.clone(),
                        };

                        self.history.insert(
                            0,
                            HistoryEntry {
                                method: self.method.as_str().to_string(),
                                url: self.url.clone(),
                                status: res.status,
                                duration_ms: res.duration_ms,
                            },
                        );
                        if self.history.len() > crate::config::HISTORY_MAX {
                            self.history.pop();
                        }

                        self.response = Some(ResponseState {
                            status: res.status,
                            status_text: res.status_text,
                            headers: res.headers,
                            body: res.body,
                            duration_ms: res.duration_ms,
                            pretty_body: pretty,
                        });
                        self.error = None;
                        self.active_tab = Tab::Response;
                    }
                    Err(e) => {
                        self.error = Some(format!("{:#}", e));
                        self.response = None;
                    }
                }
                self.loading = false;
                self.rx = None;
            }
        }
    }

    pub fn save_current(&mut self) {
        let name = format!("{} {}", self.method.as_str(), self.url);
        let req = SavedRequest {
            name: name.clone(),
            method: self.method.as_str().to_string(),
            url: self.url.clone(),
            headers: self.headers_input.clone(),
            body: self.body_input.clone(),
        };
        self.collections.push(req);

        if let Err(e) = storage::save_collections(&self.collection_path, &self.collections) {
            self.error = Some(format!("Save failed: {:#}", e));
        } else {
            self.status_message = Some(format!("💾 Saved: {}", name));
        }
    }

    pub fn load_selected_collection(&mut self) {
        if let Some(req) = self.collections.get(self.sidebar_selected) {
            self.url = req.url.clone();
            self.headers_input = req.headers.clone();
            self.body_input = req.body.clone();
            self.method = match req.method.as_str() {
                "POST" => Method::POST,
                "PUT" => Method::PUT,
                "DELETE" => Method::DELETE,
                "PATCH" => Method::PATCH,
                "HEAD" => Method::HEAD,
                "OPTIONS" => Method::OPTIONS,
                _ => Method::GET,
            };
            self.active_tab = Tab::Request;
            self.status_message = Some(format!("📂 Loaded: {}", req.name));
        }
    }

    pub fn delete_selected_collection(&mut self) {
        if self.sidebar_selected < self.collections.len() {
            let removed = self.collections.remove(self.sidebar_selected);
            if self.sidebar_selected > 0 && self.sidebar_selected >= self.collections.len() {
                self.sidebar_selected = self.collections.len().saturating_sub(1);
            }
            if let Err(e) = storage::save_collections(&self.collection_path, &self.collections) {
                self.error = Some(format!("Delete failed: {:#}", e));
            } else {
                self.status_message = Some(format!("🗑️ Deleted: {}", removed.name));
            }
        }
    }
}