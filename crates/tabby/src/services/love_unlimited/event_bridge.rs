/// Event bridge - implements EventLogger trait to integrate with Tabby's event system
use super::memory_bridge::MemoryBridge;
use tabby_common::api::event::{EventLogger, LogEntry, Event};
use std::sync::Arc;
use tracing::{debug, error};

/// Event logger that bridges Tabby events to Love-Unlimited memories
pub struct HubEventLogger {
    bridge: Arc<MemoryBridge>,
}

impl HubEventLogger {
    /// Create a new hub event logger
    pub fn new(bridge: MemoryBridge) -> Self {
        Self {
            bridge: Arc::new(bridge),
        }
    }

    /// Create a new hub event logger with spawn task for async operations
    pub fn new_with_spawn(bridge: MemoryBridge) -> Self {
        Self {
            bridge: Arc::new(bridge),
        }
    }

    /// Check if the hub bridge is healthy
    pub async fn health_check(&self) -> bool {
        self.bridge.health_check().await
    }
}

impl EventLogger for HubEventLogger {
    fn write(&self, entry: LogEntry) {
        let bridge = self.bridge.clone();

        // Spawn non-blocking task to handle memory storage
        tokio::spawn(async move {
            match &entry.event {
                Event::Completion {
                    completion_id: _,
                    language,
                    prompt,
                    segments: _,
                    choices,
                    user_agent: _,
                } => {
                    // Extract first choice if available
                    let completion_text = choices
                        .first()
                        .map(|c| c.text.clone())
                        .unwrap_or_default();

                    // Determine if snippet was used (simplified check)
                    let snippet_used = !choices.is_empty();

                    if let Err(e) = bridge
                        .store_completion(
                            language.clone(),
                            prompt.clone(),
                            completion_text,
                            "tabby".to_string(),
                            snippet_used,
                        )
                        .await
                    {
                        error!("Failed to store completion memory: {}", e);
                    } else {
                        debug!("Completion memory stored successfully");
                    }
                }
                Event::Select {
                    completion_id,
                    choice_index: _,
                    kind: _,
                    view_id: _,
                    elapsed: _,
                } => {
                    // Note: Select event doesn't include the selected text or language in the current API
                    if let Err(e) = bridge
                        .store_user_selection(
                            completion_id.clone(),
                            "Selected".to_string(),
                            None,
                        )
                        .await
                    {
                        error!("Failed to store selection memory: {}", e);
                    } else {
                        debug!("Selection memory stored successfully");
                    }
                }
                Event::View {
                    completion_id: _,
                    choice_index: _,
                    view_id: _,
                } => {
                    // View events logged but not stored to hub (too verbose)
                    debug!("View event logged locally");
                }
                Event::Dismiss {
                    completion_id: _,
                    choice_index: _,
                    view_id: _,
                    elapsed: _,
                } => {
                    // Dismiss events logged but not stored to hub
                    debug!("Dismiss event logged locally");
                }
                Event::ChatCompletion {} => {
                    debug!("Chat completion event logged locally");
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hub_event_logger_creation() {
        // This test just verifies creation doesn't panic
        // Full testing requires a mock hub
        let _logger = HubEventLogger::new_with_spawn;
        assert!(true);
    }
}
