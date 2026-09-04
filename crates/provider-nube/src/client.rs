//! Nube.SH gateway provider implementation

use async_trait::async_trait;
use hitechcloud_core::{ProviderRequest, ProviderResponse, StreamEvent};
use hitechcloud_provider_sdk::{Provider, ProviderError, ProviderResult, ResponseStream};
use hitechcloud_provider_openai::OpenAIProvider;
use hitechcloud_provider_anthropic::AnthropicProvider;
use reqwest::Client;
use serde::Deserialize;

/// Nube.SH gateway provider
pub struct NubeProvider {
    name: String,
    openai_provider: OpenAIProvider,
    anthropic_provider: AnthropicProvider,
    client: Client,
    pricing_endpoint: String,
}

/// Nube.SH pricing info
#[derive(Debug, Deserialize)]
struct NubePricing {
    model: String,
    prompt_price: f64,
    completion_price: f64,
}

impl NubeProvider {
    /// Create a new Nube.SH provider
    pub fn new(
        name: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        let api_key = api_key.into();
        let name = name.into();

        Self {
            name: name.clone(),
            openai_provider: OpenAIProvider::new(
                format!("{}-openai", name),
                "https://ai.nube-api.com",
                &api_key,
                "gpt-4",
            ),
            anthropic_provider: AnthropicProvider::new(
                format!("{}-anthropic", name),
                "https://ai.nube-api.com",
                &api_key,
                "claude-3-sonnet-20240229",
            ),
            client: Client::new(),
            pricing_endpoint: "https://ai.nube-api.com/v1/models/pricing".to_string(),
        }
    }

    /// Fetch live pricing from Nube.SH
    pub async fn fetch_pricing(&self) -> ProviderResult<Vec<NubePricing>> {
        let response = self
            .client
            .get(&self.pricing_endpoint)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ProviderError::Internal(format!(
                "Failed to fetch pricing: {}",
                response.status()
            )));
        }

        let pricing: Vec<NubePricing> = response.json().await?;
        Ok(pricing)
    }

    /// Determine which provider to use based on model name
    fn get_provider_for_model(&self, model: &str) -> &dyn Provider {
        if model.starts_with("claude") {
            &self.anthropic_provider
        } else {
            &self.openai_provider
        }
    }
}

#[async_trait]
impl Provider for NubeProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn provider_type(&self) -> &str {
        "nube-gateway"
    }

    async fn is_available(&self) -> bool {
        self.openai_provider.is_available().await
    }

    async fn list_models(&self) -> ProviderResult<Vec<String>> {
        let mut models = self.openai_provider.list_models().await?;
        models.extend(self.anthropic_provider.list_models().await?);
        Ok(models)
    }

    async fn complete(&self, request: ProviderRequest) -> ProviderResult<ProviderResponse> {
        let provider = self.get_provider_for_model(&request.model);
        provider.complete(request).await
    }

    async fn complete_stream(&self, request: ProviderRequest) -> ProviderResult<ResponseStream> {
        let provider = self.get_provider_for_model(&request.model);
        provider.complete_stream(request).await
    }

    fn max_context_tokens(&self, model: &str) -> Option<u32> {
        let provider = self.get_provider_for_model(model);
        provider.max_context_tokens(model)
    }

    fn cost_per_token(&self, model: &str) -> Option<(f64, f64)> {
        let provider = self.get_provider_for_model(model);
        provider.cost_per_token(model)
    }
}
