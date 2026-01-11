/// HTTP client for communicating with Love-Unlimited Hub
use super::models::{HubConfig, HubError, HubResult, RecallQuery, RecallResponse, RememberRequest, RememberResponse};
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tracing::{debug, error, warn};

/// HTTP client for Love-Unlimited Hub
#[derive(Clone)]
pub struct HubClient {
    client: Client,
    config: HubConfig,
}

impl HubClient {
    /// Create a new hub client from configuration
    pub fn new(config: HubConfig) -> HubResult<Self> {
        config.validate().map_err(|e| HubError::InvalidConfig(e))?;

        let timeout = Duration::from_secs(config.timeout_secs);
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| HubError::HttpError(e.to_string()))?;

        Ok(Self { client, config })
    }

    /// Check if hub is available and healthy
    pub async fn health_check(&self) -> HubResult<bool> {
        if !self.config.enabled {
            return Ok(false);
        }

        let url = format!("{}/health", self.config.hub_url);
        match self.client.get(&url).send().await {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(e) => {
                warn!("Hub health check failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Store a memory in the hub
    pub async fn remember(&self, request: RememberRequest) -> HubResult<String> {
        if !self.config.enabled {
            debug!("Love-Unlimited integration disabled, skipping remember");
            return Ok(String::new());
        }

        let url = format!("{}/remember", self.config.hub_url);
        debug!("Storing memory in hub: {}", url);

        let response = self
            .client
            .post(&url)
            .header("X-API-Key", &self.config.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send remember request: {}", e);
                if e.is_timeout() {
                    HubError::Timeout
                } else {
                    HubError::HttpError(e.to_string())
                }
            })?;

        match response.status() {
            StatusCode::OK => {
                let body: RememberResponse = response.json().await.map_err(|e| {
                    error!("Failed to parse remember response: {}", e);
                    HubError::SerializationError(e.to_string())
                })?;

                if let Some(data) = body.data {
                    debug!("Memory stored successfully: {}", data.memory_id);
                    Ok(data.memory_id)
                } else {
                    Err(HubError::Unknown("No memory_id in response".to_string()))
                }
            }
            StatusCode::UNAUTHORIZED => {
                error!("Authentication failed with hub");
                Err(HubError::AuthFailed)
            }
            status => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                error!("Hub returned error {}: {}", status, error_text);
                Err(HubError::HttpError(format!("Status {}: {}", status, error_text)))
            }
        }
    }

    /// Recall memories from the hub
    pub async fn recall(&self, query: RecallQuery) -> HubResult<RecallResponse> {
        if !self.config.enabled {
            debug!("Love-Unlimited integration disabled, skipping recall");
            return Ok(RecallResponse {
                memories: vec![],
                count: 0,
            });
        }

        let url = format!("{}/recall", self.config.hub_url);
        debug!("Recalling memories from hub: query={}", query.q);

        let response = self
            .client
            .get(&url)
            .header("X-API-Key", &self.config.api_key)
            .query(&[
                ("q", &query.q),
                ("limit", &query.limit.to_string()),
            ])
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send recall request: {}", e);
                if e.is_timeout() {
                    HubError::Timeout
                } else {
                    HubError::HttpError(e.to_string())
                }
            })?;

        match response.status() {
            StatusCode::OK => {
                let body: RecallResponse = response.json().await.map_err(|e| {
                    error!("Failed to parse recall response: {}", e);
                    HubError::SerializationError(e.to_string())
                })?;
                debug!("Retrieved {} memories from hub", body.count);
                Ok(body)
            }
            StatusCode::UNAUTHORIZED => {
                error!("Authentication failed with hub");
                Err(HubError::AuthFailed)
            }
            status => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                error!("Hub returned error {}: {}", status, error_text);
                Err(HubError::HttpError(format!("Status {}: {}", status, error_text)))
            }
        }
    }

    /// Get the hub configuration
    pub fn config(&self) -> &HubConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation_disabled() {
        let config = HubConfig::disabled();
        let result = HubClient::new(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_remember_request_serialization() {
        let request = RememberRequest::new(
            "Test completion".to_string(),
            "learning".to_string(),
        );
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"content\""));
        assert!(json.contains("\"type\""));
    }
}
