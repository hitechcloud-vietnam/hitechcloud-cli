//! Anthropic Claude provider implementation

use async_trait::async_trait;
use hitechcloud_core::{ProviderRequest, ProviderResponse, StreamEvent};
use hitechcloud_provider_sdk::{Provider, ProviderError, ProviderResult, ResponseStream};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use futures::StreamExt;

/// Anthropic Claude provider
pub struct AnthropicProvider {
    name: String,
    endpoint: String,
    api_key: String,
    default_model: String,
    client: Client,
}

/// Anthropic API request format
#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<serde_json::Value>>,
}

/// Anthropic message format
#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

/// Anthropic API response format
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    id: String,
    model: String,
    content: Vec<AnthropicContent>,
    usage: AnthropicUsage,
    stop_reason: Option<String>,
}

/// Anthropic content block
#[derive(Debug, Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

/// Anthropic usage
#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider
    pub fn new(
        name: impl Into<String>,
        endpoint: impl Into<String>,
        api_key: impl Into<String>,
        default_model: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            endpoint: endpoint.into(),
            api_key: api_key.into(),
            default_model: default_model.into(),
            client: Client::new(),
        }
    }

    /// Convert HiTechCloud request to Anthropic format
    fn convert_request(&self, request: ProviderRequest) -> AnthropicRequest {
        let mut system = None;
        let mut messages = Vec::new();

        for msg in request.messages {
            match msg.role {
                hitechcloud_core::Role::System => {
                    system = Some(msg.content);
                }
                _ => {
                    messages.push(AnthropicMessage {
                        role: format!("{:?}", msg.role).to_lowercase(),
                        content: msg.content,
                    });
                }
            }
        }

        AnthropicRequest {
            model: request.model,
            messages,
            max_tokens: request.max_tokens.unwrap_or(4096),
            system,
            temperature: request.temperature,
            stream: request.stream,
            tools: request.tools.map(|tools| {
                tools
                    .into_iter()
                    .map(|tool| {
                        serde_json::json!({
                            "name": tool.function.name,
                            "description": tool.function.description,
                            "input_schema": tool.function.parameters,
                        })
                    })
                    .collect()
            }),
        }
    }

    /// Convert Anthropic response to HiTechCloud format
    fn convert_response(&self, response: AnthropicResponse) -> ProviderResponse {
        let content = response
            .content
            .into_iter()
            .filter_map(|c| c.text)
            .collect::<Vec<_>>()
            .join("");

        let message = hitechcloud_core::Message {
            id: response.id.clone(),
            role: hitechcloud_core::Role::Assistant,
            content,
            tool_calls: None,
            tool_call_id: None,
            timestamp: chrono::Utc::now(),
        };

        ProviderResponse {
            id: response.id,
            model: response.model,
            message,
            usage: hitechcloud_core::Usage {
                prompt_tokens: response.usage.input_tokens,
                completion_tokens: response.usage.output_tokens,
                total_tokens: response.usage.input_tokens + response.usage.output_tokens,
            },
            truncated: response.stop_reason.as_deref() == Some("max_tokens"),
        }
    }
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn provider_type(&self) -> &str {
        "anthropic-compatible"
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn list_models(&self) -> ProviderResult<Vec<String>> {
        // Anthropic doesn't have a list models endpoint, return known models
        Ok(vec![
            "claude-3-opus-20240229".to_string(),
            "claude-3-sonnet-20240229".to_string(),
            "claude-3-haiku-20240307".to_string(),
            "claude-2.1".to_string(),
            "claude-2.0".to_string(),
            "claude-instant-1.2".to_string(),
        ])
    }

    async fn complete(&self, request: ProviderRequest) -> ProviderResult<ProviderResponse> {
        let anthropic_request = self.convert_request(request);

        let url = format!("{}/v1/messages", self.endpoint);
        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&anthropic_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::Internal(format!(
                "API error: {}",
                error_text
            )));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;
        Ok(self.convert_response(anthropic_response))
    }

    async fn complete_stream(&self, request: ProviderRequest) -> ProviderResult<ResponseStream> {
        let anthropic_request = self.convert_request(request);

        let url = format!("{}/v1/messages", self.endpoint);
        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&anthropic_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::Internal(format!(
                "API error: {}",
                error_text
            )));
        }

        let stream = response.bytes_stream();
        let processed = stream.map(|chunk| {
            match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    // Parse SSE format
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let data = &line[6..];
                            if let Ok(event) = serde_json::from_str::<StreamEvent>(data) {
                                return Ok(event);
                            }
                        }
                    }
                    Err(ProviderError::Internal("Failed to parse stream event".to_string()))
                }
                Err(e) => Err(ProviderError::Network(e)),
            }
        });

        Ok(Box::pin(processed))
    }

    fn max_context_tokens(&self, _model: &str) -> Option<u32> {
        Some(200000) // Claude 3 context window
    }

    fn cost_per_token(&self, _model: &str) -> Option<(f64, f64)> {
        Some((0.000015, 0.000075)) // Claude 3 Sonnet pricing
    }
}
