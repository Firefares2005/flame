use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SavedRequest {
    pub name: String,
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub headers: String,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct CollectionsFile {
    #[serde(default)]
    requests: Vec<SavedRequest>,
}

pub fn load_collections(path: &str) -> Result<Vec<SavedRequest>> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))?;
    if content.trim().is_empty() {
        return Ok(Vec::new());
    }
    let parsed: CollectionsFile = toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path))?;
    Ok(parsed.requests)
}

pub fn save_collections(path: &str, reqs: &[SavedRequest]) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create dir {:?}", parent))?;
        }
    }
    let file = CollectionsFile {
        requests: reqs.to_vec(),
    };
    let toml_str = toml::to_string_pretty(&file).context("Failed to serialize TOML")?;
    fs::write(path, toml_str).with_context(|| format!("Failed to write {}", path))?;
    Ok(())
}