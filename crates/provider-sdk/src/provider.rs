//! Provider trait definition

use async_trait::async_trait;
use hitechcloud_core::{ProviderRequest, ProviderResponse, StreamEvent};
use std::pin::Pin;
use futures::Stream;

/// Error type for provider operations
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for provider operations
pub type ProviderResult<T> = std::result::Result<T, ProviderError>;

/// Stream type for streaming responses
pub type ResponseStream = Pin<Box<dyn Stream<Item = ProviderResult<StreamEvent>> + Send>>;

/// Core Provider trait that all LLM providers must implement
#[async_trait]
pub trait Provider: Send + Sync {
    /// Get the provider name
    fn name(&self) -> &str;

    /// Get the provider type (e.g., "openai-compatible", "anthropic-compatible")
    fn provider_type(&self) -> &str;

    /// Check if the provider is available (has valid credentials, etc.)
    async fn is_available(&self) -> bool;

    /// Get available models from this provider
    async fn list_models(&self) -> ProviderResult<Vec<String>>;

    /// Send a completion request and get a response
    async fn complete(&self, request: ProviderRequest) -> ProviderResult<ProviderResponse>;

    /// Send a completion request and get a streaming response
    async fn complete_stream(&self, request: ProviderRequest) -> ProviderResult<ResponseStream>;

    /// Get the maximum context window for a model
    fn max_context_tokens(&self, model: &str) -> Option<u32>;

    /// Get the cost per token for a model (prompt, completion)
    fn cost_per_token(&self, model: &str) -> Option<(f64, f64)>;
}
