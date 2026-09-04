//! Core types for HiTechCloud CLI

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Message role in a conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

/// A message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message ID
    pub id: String,

    /// Message role
    pub role: Role,

    /// Message content
    pub content: String,

    /// Tool calls (if any)
    pub tool_calls: Option<Vec<ToolCall>>,

    /// Tool call ID (for tool responses)
    pub tool_call_id: Option<String>,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// A tool call made by the assistant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Tool call ID
    pub id: String,

    /// Tool type (usually "function")
    #[serde(rename = "type")]
    pub call_type: String,

    /// Function details
    pub function: FunctionCall,
}

/// Function call details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// Function name
    pub name: String,

    /// Function arguments (JSON string)
    pub arguments: String,
}

/// A tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool type (usually "function")
    #[serde(rename = "type")]
    pub tool_type: String,

    /// Function definition
    pub function: FunctionDefinition,
}

/// Function definition for a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    /// Function name
    pub name: String,

    /// Function description
    pub description: String,

    /// Function parameters schema (JSON Schema)
    pub parameters: serde_json::Value,
}

/// Provider request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRequest {
    /// Model to use
    pub model: String,

    /// Messages in the conversation
    pub messages: Vec<Message>,

    /// Tools available
    pub tools: Option<Vec<ToolDefinition>>,

    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,

    /// Temperature for generation
    pub temperature: Option<f32>,

    /// Whether to stream the response
    pub stream: bool,
}

/// Provider response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderResponse {
    /// Response ID
    pub id: String,

    /// Model used
    pub model: String,

    /// Generated message
    pub message: Message,

    /// Usage statistics
    pub usage: Usage,

    /// Whether the response was truncated
    pub truncated: bool,
}

/// Usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Tokens in the prompt
    pub prompt_tokens: u32,

    /// Tokens in the completion
    pub completion_tokens: u32,

    /// Total tokens
    pub total_tokens: u32,
}

/// Streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    /// Event type
    pub event: StreamEventType,

    /// Event data
    pub data: StreamEventData,
}

/// Stream event type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamEventType {
    MessageStart,
    ContentDelta,
    ToolCallStart,
    ToolCallDelta,
    MessageEnd,
    Error,
}

/// Stream event data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamEventData {
    MessageStart {
        id: String,
        model: String,
        role: Role,
    },
    ContentDelta {
        delta: ContentDelta,
    },
    ToolCallStart {
        index: u32,
        id: String,
        function: FunctionCall,
    },
    ToolCallDelta {
        index: u32,
        delta: ToolCallDelta,
    },
    MessageEnd {
        usage: Usage,
    },
    Error {
        message: String,
        code: Option<String>,
    },
}

/// Content delta for streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentDelta {
    /// Delta content
    pub content: String,
}

/// Tool call delta for streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDelta {
    /// Delta arguments
    pub arguments: String,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Tool call ID
    pub tool_call_id: String,

    /// Result content
    pub content: String,

    /// Whether the tool execution was successful
    pub success: bool,

    /// Error message (if any)
    pub error: Option<String>,
}

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    Idle,
    Thinking,
    ExecutingTool,
    WaitingForApproval,
    Completed,
    Failed,
    Cancelled,
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Session ID
    pub id: Uuid,

    /// Session name
    pub name: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,

    /// Number of messages
    pub message_count: usize,

    /// Total tokens used
    pub total_tokens: u64,

    /// Total cost in USD
    pub total_cost_usd: f64,
}

impl Message {
    /// Create a new user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: Role::User,
            content: content.into(),
            tool_calls: None,
            tool_call_id: None,
            timestamp: Utc::now(),
        }
    }

    /// Create a new assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: Role::Assistant,
            content: content.into(),
            tool_calls: None,
            tool_call_id: None,
            timestamp: Utc::now(),
        }
    }

    /// Create a new system message
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: Role::System,
            content: content.into(),
            tool_calls: None,
            tool_call_id: None,
            timestamp: Utc::now(),
        }
    }

    /// Create a new tool response message
    pub fn tool(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            role: Role::Tool,
            content: content.into(),
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
            timestamp: Utc::now(),
        }
    }
}
