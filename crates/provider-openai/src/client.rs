//! OpenAI-compatible provider implementation

use async_trait::async_trait;
use hitechcloud_core::{ProviderRequest, ProviderResponse, StreamEvent};
use hitechcloud_provider_sdk::{Provider, ProviderError, ProviderResult, ResponseStream};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use futures::StreamExt;

/// OpenAI-compatible provider
pub struct OpenAIProvider {
    name: String,
    endpoint: String,
    api_key: String,
    default_model: String,
    client: Client,
}

/// OpenAI API request format
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<serde_json::Value>>,
}

/// OpenAI message format
#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

/// OpenAI API response format
#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    id: String,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: OpenAIUsage,
}

/// OpenAI choice
#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
    finish_reason: Option<String>,
}

/// OpenAI usage
#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl OpenAIProvider {
    /// Create a new OpenAI-compatible provider
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

    /// Convert HiTechCloud request to OpenAI format
    fn convert_request(&self, request: ProviderRequest) -> OpenAIRequest {
        let messages = request
            .messages
            .into_iter()
            .map(|msg| OpenAIMessage {
                role: format!("{:?}", msg.role).to_lowercase(),
                content: msg.content,
                tool_calls: msg.tool_calls.map(|tc| {
                    tc.into_iter()
                        .map(|call| serde_json::json!({
                            "id": call.id,
                            "type": call.call_type,
                            "function": {
                                "name": call.function.name,
                                "arguments": call.function.arguments,
                            }
                        }))
                        .collect()
                }),
                tool_call_id: msg.tool_call_id,
            })
            .collect();

        OpenAIRequest {
            model: request.model,
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            stream: request.stream,
            tools: request.tools.map(|tools| {
                tools
                    .into_iter()
                    .map(|tool| {
                        serde_json::json!({
                            "type": tool.tool_type,
                            "function": {
                                "name": tool.function.name,
                                "description": tool.function.description,
                                "parameters": tool.function.parameters,
                            }
                        })
                    })
                    .collect()
            }),
        }
    }

    /// Convert OpenAI response to HiTechCloud format
    fn convert_response(&self, response: OpenAIResponse, model: &str) -> ProviderResponse {
        let choice = response.choices.into_iter().next().unwrap();
        let message = hitechcloud_core::Message {
            id: response.id.clone(),
            role: match choice.message.role.as_str() {
                "assistant" => hitechcloud_core::Role::Assistant,
                "user" => hitechcloud_core::Role::User,
                "system" => hitechcloud_core::Role::System,
                "tool" => hitechcloud_core::Role::Tool,
                _ => hitechcloud_core::Role::Assistant,
            },
            content: choice.message.content,
            tool_calls: None,
            tool_call_id: choice.message.tool_call_id,
            timestamp: chrono::Utc::now(),
        };

        ProviderResponse {
            id: response.id,
            model: model.to_string(),
            message,
            usage: hitechcloud_core::Usage {
                prompt_tokens: response.usage.prompt_tokens,
                completion_tokens: response.usage.completion_tokens,
                total_tokens: response.usage.total_tokens,
            },
            truncated: choice.finish_reason.as_deref() == Some("length"),
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn provider_type(&self) -> &str {
        "openai-compatible"
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn list_models(&self) -> ProviderResult<Vec<String>> {
        let url = format!("{}/v1/models", self.endpoint);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ProviderError::Internal(format!(
                "Failed to list models: {}",
                response.status()
            )));
        }

        let body: serde_json::Value = response.json().await?;
        let models = body["data"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
            .collect();

        Ok(models)
    }

    async fn complete(&self, request: ProviderRequest) -> ProviderResult<ProviderResponse> {
        let model = request.model.clone();
        let openai_request = self.convert_request(request);

        let url = format!("{}/v1/chat/completions", self.endpoint);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&openai_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::Internal(format!(
                "API error: {}",
                error_text
            )));
        }

        let openai_response: OpenAIResponse = response.json().await?;
        Ok(self.convert_response(openai_response, &model))
    }

    async fn complete_stream(&self, request: ProviderRequest) -> ProviderResult<ResponseStream> {
        let openai_request = self.convert_request(request);

        let url = format!("{}/v1/chat/completions", self.endpoint);
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&openai_request)
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
                            if data == "[DONE]" {
                                continue;
                            }
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
        Some(128000) // Default for GPT-4
    }

    fn cost_per_token(&self, _model: &str) -> Option<(f64, f64)> {
        Some((0.00003, 0.00006)) // Default GPT-4 pricing
    }
}
