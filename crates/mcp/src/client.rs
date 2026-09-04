//! MCP client for connecting to MCP servers

use serde::{Deserialize, Serialize};

/// MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub endpoint: String,
    pub transport: McpTransport,
}

/// MCP transport type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum McpTransport {
    Stdio,
    Http,
    Sse,
}

/// MCP client error
#[derive(Debug, thiserror::Error)]
pub enum McpClientError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// MCP client for connecting to MCP servers
pub struct McpClient {
    config: McpServerConfig,
}

impl McpClient {
    /// Create a new McpClient
    pub fn new(config: McpServerConfig) -> Self {
        Self { config }
    }

    /// Connect to the MCP server
    pub async fn connect(&self) -> Result<(), McpClientError> {
        // TODO: Implement MCP connection
        tracing::info!("Connecting to MCP server: {}", self.config.name);
        Ok(())
    }

    /// Send a request to the MCP server
    pub async fn send_request(&self, _method: &str, _params: serde_json::Value) -> Result<serde_json::Value, McpClientError> {
        // TODO: Implement MCP request
        Ok(serde_json::json!({}))
    }
}
