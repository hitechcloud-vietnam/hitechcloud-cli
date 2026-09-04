//! Agent implementation with ReAct loop

use hitechcloud_core::{Message, Role, AgentStatus, ToolResult, ToolCall, FunctionCall};
use hitechcloud_provider_sdk::Provider;
use std::sync::Arc;

/// Agent error type
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Tool execution error: {0}")]
    ToolExecution(String),

    #[error("Max iterations reached")]
    MaxIterationsReached,

    #[error("Cancelled by user")]
    Cancelled,

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for agent operations
pub type AgentResult<T> = std::result::Result<T, AgentError>;

/// Agent configuration
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub max_iterations: usize,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub streaming: bool,
    pub model: String,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            max_tokens: Some(4096),
            temperature: Some(0.7),
            streaming: true,
            model: "gpt-4".to_string(),
        }
    }
}

/// Tool trait for executing tools
#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    /// Get the tool name
    fn name(&self) -> &str;

    /// Get the tool description
    fn description(&self) -> &str;

    /// Get the tool parameters schema (JSON Schema)
    fn parameters(&self) -> serde_json::Value;

    /// Execute the tool
    async fn execute(&self, arguments: &str) -> Result<String, String>;
}

/// The main Agent struct implementing ReAct loop
pub struct Agent {
    config: AgentConfig,
    provider: Arc<dyn Provider>,
    messages: Vec<Message>,
    status: AgentStatus,
    tools: Vec<Arc<dyn Tool>>,
}

impl Agent {
    /// Create a new Agent
    pub fn new(provider: Arc<dyn Provider>, config: AgentConfig) -> Self {
        Self {
            config,
            provider,
            messages: Vec::new(),
            status: AgentStatus::Idle,
            tools: Vec::new(),
        }
    }

    /// Add a tool to the agent
    pub fn add_tool(&mut self, tool: Arc<dyn Tool>) {
        self.tools.push(tool);
    }

    /// Get current agent status
    pub fn status(&self) -> &AgentStatus {
        &self.status
    }

    /// Get message history
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Add a system message
    pub fn set_system_message(&mut self, content: impl Into<String>) {
        self.messages.insert(0, Message::system(content));
    }

    /// Run the agent with a user message
    pub async fn run(&mut self, user_message: impl Into<String>) -> AgentResult<String> {
        self.messages.push(Message::user(user_message));
        self.status = AgentStatus::Thinking;

        for iteration in 0..self.config.max_iterations {
            tracing::debug!("Agent iteration {}", iteration + 1);

            // Build tool definitions
            let tool_definitions = if !self.tools.is_empty() {
                Some(self.tools.iter().map(|tool| {
                    hitechcloud_core::ToolDefinition {
                        tool_type: "function".to_string(),
                        function: hitechcloud_core::FunctionDefinition {
                            name: tool.name().to_string(),
                            description: tool.description().to_string(),
                            parameters: tool.parameters(),
                        },
                    }
                }).collect())
            } else {
                None
            };

            let request = hitechcloud_core::ProviderRequest {
                model: self.config.model.clone(),
                messages: self.messages.clone(),
                tools: tool_definitions,
                max_tokens: self.config.max_tokens,
                temperature: self.config.temperature,
                stream: self.config.streaming,
            };

            let response = self
                .provider
                .complete(request)
                .await
                .map_err(|e| AgentError::Provider(e.to_string()))?;

            self.messages.push(response.message.clone());

            // Check if we have tool calls to execute
            if let Some(tool_calls) = &response.message.tool_calls {
                self.status = AgentStatus::ExecutingTool;

                for tool_call in tool_calls {
                    let result = self.execute_tool(tool_call).await;
                    let tool_message = match result {
                        Ok(content) => Message::tool(&tool_call.id, content),
                        Err(e) => Message::tool(&tool_call.id, format!("Error: {}", e)),
                    };
                    self.messages.push(tool_message);
                }
            } else {
                // No tool calls, we're done
                self.status = AgentStatus::Completed;
                return Ok(response.message.content);
            }
        }

        self.status = AgentStatus::Failed;
        Err(AgentError::MaxIterationsReached)
    }

    /// Execute a tool call
    async fn execute_tool(&self, tool_call: &ToolCall) -> Result<String, AgentError> {
        let tool = self.tools.iter().find(|t| t.name() == tool_call.function.name);

        match tool {
            Some(tool) => {
                tool.execute(&tool_call.function.arguments)
                    .await
                    .map_err(|e| AgentError::ToolExecution(e))
            }
            None => Err(AgentError::ToolExecution(format!(
                "Tool not found: {}",
                tool_call.function.name
            ))),
        }
    }

    /// Cancel the agent
    pub fn cancel(&mut self) {
        self.status = AgentStatus::Cancelled;
    }
}
