//! Agent daemon for VS Code extension communication

use serde::{Deserialize, Serialize};

/// Daemon configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    pub socket_path: String,
    pub port: Option<u16>,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            socket_path: "/tmp/hitechcloud-agentd.sock".to_string(),
            port: None,
        }
    }
}

/// Agent daemon error
#[derive(Debug, thiserror::Error)]
pub enum DaemonError {
    #[error("Socket error: {0}")]
    Socket(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Agent daemon for VS Code extension communication
pub struct AgentDaemon {
    config: DaemonConfig,
}

impl AgentDaemon {
    /// Create a new AgentDaemon
    pub fn new(config: DaemonConfig) -> Self {
        Self { config }
    }

    /// Start the daemon
    pub async fn start(&self) -> Result<(), DaemonError> {
        // TODO: Implement daemon startup
        tracing::info!("Starting agent daemon on {}", self.config.socket_path);
        Ok(())
    }

    /// Stop the daemon
    pub async fn stop(&self) -> Result<(), DaemonError> {
        // TODO: Implement daemon shutdown
        tracing::info!("Stopping agent daemon");
        Ok(())
    }
}
