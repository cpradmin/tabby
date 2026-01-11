/// Type definitions and serde models for Love-Unlimited Hub integration
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for Love-Unlimited Hub connection
#[derive(Debug, Clone)]
pub struct HubConfig {
    pub enabled: bool,
    pub hub_url: String,
    pub api_key: String,
    pub timeout_secs: u64,
    pub max_retries: u32,
    pub features: HubFeatures,
}

#[derive(Debug, Clone)]
pub struct HubFeatures {
    pub log_completions: bool,
    pub log_user_events: bool,
    pub track_errors: bool,
    pub enrich_context: bool,
}

impl Default for HubFeatures {
    fn default() -> Self {
        Self {
            log_completions: true,
            log_user_events: true,
            track_errors: true,
            enrich_context: false,
        }
    }
}

/// Request payload for storing memories in the hub
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RememberRequest {
    pub content: String,
    #[serde(rename = "type")]
    pub memory_type: String,
    pub significance: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub private: bool,
}

impl RememberRequest {
    pub fn new(content: String, memory_type: String) -> Self {
        Self {
            content,
            memory_type,
            significance: "medium".to_string(),
            tags: vec![],
            metadata: HashMap::new(),
            private: true,
        }
    }
}

/// Response from hub memory storage
#[derive(Debug, Deserialize)]
pub struct RememberResponse {
    pub message: Option<String>,
    pub data: Option<RememberData>,
}

#[derive(Debug, Deserialize)]
pub struct RememberData {
    pub memory_id: String,
}

/// Query parameters for recalling memories
#[derive(Debug, Clone, Serialize)]
pub struct RecallQuery {
    pub q: String,
    pub limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl RecallQuery {
    pub fn new(query: String, limit: u32) -> Self {
        Self {
            q: query,
            limit,
            type_: None,
        }
    }
}

/// Response from hub memory recall
#[derive(Debug, Deserialize)]
pub struct RecallResponse {
    pub memories: Vec<Memory>,
    pub count: u32,
}

#[derive(Debug, Deserialize)]
pub struct Memory {
    pub memory_id: String,
    pub content: String,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub memory_type: String,
    pub significance: String,
    pub tags: Vec<String>,
}

/// Error types for hub operations
#[derive(Debug, thiserror::Error)]
pub enum HubError {
    #[error("HTTP error: {0}")]
    HttpError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Timeout")]
    Timeout,

    #[error("Hub unavailable")]
    Unavailable,

    #[error("Authentication failed")]
    AuthFailed,

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for hub operations
pub type HubResult<T> = Result<T, HubError>;
