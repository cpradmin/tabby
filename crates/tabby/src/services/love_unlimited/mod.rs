/// Love-Unlimited Hub integration module
///
/// Provides memory storage and retrieval capabilities for Tabby completions
/// and events, integrated with the Love-Unlimited sovereign memory hub.
///
/// # Features
/// - Automatic completion logging to Love-Unlimited
/// - Error tracking and analysis
/// - User event recording (selections, dismissals)
/// - Non-blocking async operations
/// - Graceful degradation if hub unavailable
///
/// # Configuration
/// Set these environment variables to enable:
/// - `LOVE_UNLIMITED_ENABLED`: true/false
/// - `LOVE_UNLIMITED_URL`: http://localhost:9003 (default)
/// - `LOVE_UNLIMITED_KEY`: Your API key for the hub
/// - `LOVE_UNLIMITED_TIMEOUT`: Timeout in seconds (default: 5)

pub mod client;
pub mod config;
pub mod event_bridge;
pub mod memory_bridge;
pub mod models;

pub use client::HubClient;
pub use config::*;
pub use event_bridge::HubEventLogger;
pub use memory_bridge::MemoryBridge;
pub use models::{HubConfig, HubError, HubFeatures, HubResult};
