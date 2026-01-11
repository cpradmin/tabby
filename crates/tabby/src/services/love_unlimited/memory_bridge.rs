/// Memory bridge - abstraction for storing and retrieving memories from hub
use super::client::HubClient;
use super::models::{HubResult, RememberRequest, RecallQuery};
use serde_json::json;
use std::sync::Arc;
use tracing::debug;

/// Bridge for managing memories in the hub
#[derive(Clone)]
pub struct MemoryBridge {
    client: Arc<HubClient>,
}

impl MemoryBridge {
    /// Create a new memory bridge with a hub client
    pub fn new(client: HubClient) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    /// Store a completion event as a learning memory
    pub async fn store_completion(
        &self,
        language: String,
        prompt: String,
        completion_text: String,
        model_name: String,
        snippet_used: bool,
    ) -> HubResult<String> {
        let truncated_prompt = if prompt.len() > 200 {
            format!("{}...", &prompt[..197])
        } else {
            prompt.clone()
        };

        let content = format!(
            "Completion in {}: {}",
            language, truncated_prompt
        );

        let mut request = RememberRequest::new(content, "learning".to_string());
        request.significance = if snippet_used { "high" } else { "medium" }.to_string();
        request.tags = vec![
            language.clone(),
            "completion".to_string(),
            model_name,
        ];

        // Add metadata
        request.metadata.insert("language".to_string(), json!(language));
        request.metadata.insert("prompt_length".to_string(), json!(prompt.len()));
        request.metadata.insert("completion_length".to_string(), json!(completion_text.len()));
        request.metadata.insert("snippet_used".to_string(), json!(snippet_used));

        debug!("Storing completion memory: {} chars prompt", prompt.len());
        self.client.remember(request).await
    }

    /// Store an error event as an insight memory
    pub async fn store_error(
        &self,
        error_message: String,
        language: Option<String>,
        context: Option<String>,
    ) -> HubResult<String> {
        let content = format!(
            "Completion error in {}: {}",
            language.clone().unwrap_or_else(|| "unknown".to_string()),
            error_message
        );

        let mut request = RememberRequest::new(content, "insight".to_string());
        request.significance = "high".to_string();
        request.tags = vec!["error".to_string(), "completion-failure".to_string()];

        if let Some(lang) = language {
            request.tags.push(lang.clone());
            request.metadata.insert("language".to_string(), json!(lang));
        }

        if let Some(ctx) = context {
            request.metadata.insert("context".to_string(), json!(ctx));
        }

        debug!("Storing error memory: {}", error_message);
        self.client.remember(request).await
    }

    /// Store a user selection event as a decision memory
    pub async fn store_user_selection(
        &self,
        completion_id: String,
        selected_text: String,
        language: Option<String>,
    ) -> HubResult<String> {
        let content = format!("User selected completion in {}",
            language.as_ref().unwrap_or(&"unknown".to_string()));

        let mut request = RememberRequest::new(content, "decision".to_string());
        request.significance = "medium".to_string();
        request.tags = vec!["user-selection".to_string()];

        if let Some(lang) = language {
            request.tags.push(lang.clone());
            request.metadata.insert("language".to_string(), json!(lang));
        }

        request.metadata.insert("completion_id".to_string(), json!(completion_id));
        request.metadata.insert("selected_length".to_string(), json!(selected_text.len()));

        debug!("Storing user selection memory");
        self.client.remember(request).await
    }

    /// Recall memories for context enrichment
    pub async fn recall_context(
        &self,
        language: Option<String>,
        limit: u32,
    ) -> HubResult<Vec<String>> {
        let query = if let Some(lang) = language {
            format!("{} completions patterns insights", lang)
        } else {
            "completion patterns insights".to_string()
        };

        let recall_query = RecallQuery::new(query, limit);
        let response = self.client.recall(recall_query).await?;

        let contexts = response
            .memories
            .iter()
            .map(|m| m.content.clone())
            .collect();

        debug!("Recalled {} context memories", response.count);
        Ok(contexts)
    }

    /// Check if the hub is healthy
    pub async fn health_check(&self) -> bool {
        match self.client.health_check().await {
            Ok(healthy) => healthy,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remember_request_building() {
        let req = RememberRequest::new(
            "Test".to_string(),
            "learning".to_string(),
        );
        assert_eq!(req.memory_type, "learning");
        assert_eq!(req.content, "Test");
        assert!(req.private);
    }
}
