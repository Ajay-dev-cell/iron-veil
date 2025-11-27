use serde::Deserialize;
use std::fs;
use anyhow::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub rules: Vec<MaskingRule>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MaskingRule {
    pub table: Option<String>,
    pub column: String,
    pub strategy: String,
}

impl AppConfig {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: AppConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}
