/// Configuration loading and management for Love-Unlimited Hub
use super::models::{HubConfig, HubFeatures};
use std::env;
use tracing::{info, warn};

impl HubConfig {
    /// Load configuration from environment variables and defaults
    pub fn from_env() -> Self {
        let enabled = env::var("LOVE_UNLIMITED_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        if !enabled {
            info!("Love-Unlimited integration disabled");
            return Self::disabled();
        }

        let hub_url = env::var("LOVE_UNLIMITED_URL")
            .unwrap_or_else(|_| "http://localhost:9003".to_string());

        let api_key = match env::var("LOVE_UNLIMITED_KEY") {
            Ok(key) => key,
            Err(_) => {
                warn!("LOVE_UNLIMITED_KEY environment variable not set");
                "".to_string()
            }
        };

        let timeout_secs = env::var("LOVE_UNLIMITED_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);

        let max_retries = env::var("LOVE_UNLIMITED_MAX_RETRIES")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3);

        let features = HubFeatures {
            log_completions: env::var("LOVE_UNLIMITED_LOG_COMPLETIONS")
                .unwrap_or_else(|_| "true".to_string())
                .parse::<bool>()
                .unwrap_or(true),
            log_user_events: env::var("LOVE_UNLIMITED_LOG_USER_EVENTS")
                .unwrap_or_else(|_| "true".to_string())
                .parse::<bool>()
                .unwrap_or(true),
            track_errors: env::var("LOVE_UNLIMITED_TRACK_ERRORS")
                .unwrap_or_else(|_| "true".to_string())
                .parse::<bool>()
                .unwrap_or(true),
            enrich_context: env::var("LOVE_UNLIMITED_ENRICH_CONTEXT")
                .unwrap_or_else(|_| "false".to_string())
                .parse::<bool>()
                .unwrap_or(false),
        };

        Self {
            enabled,
            hub_url,
            api_key,
            timeout_secs,
            max_retries,
            features,
        }
    }

    /// Create a disabled configuration
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            hub_url: String::new(),
            api_key: String::new(),
            timeout_secs: 5,
            max_retries: 3,
            features: HubFeatures::default(),
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        if self.hub_url.is_empty() {
            return Err("hub_url is empty".to_string());
        }

        if self.api_key.is_empty() {
            return Err("api_key is empty - set LOVE_UNLIMITED_KEY environment variable".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        env::set_var("LOVE_UNLIMITED_ENABLED", "true");
        env::set_var("LOVE_UNLIMITED_URL", "http://localhost:9003");
        env::set_var("LOVE_UNLIMITED_KEY", "test_key");

        let config = HubConfig::from_env();
        assert!(config.enabled);
        assert_eq!(config.hub_url, "http://localhost:9003");
        assert_eq!(config.api_key, "test_key");
    }

    #[test]
    fn test_config_disabled() {
        let config = HubConfig::disabled();
        assert!(!config.enabled);
    }
}
