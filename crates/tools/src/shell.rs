//! Shell command execution tool

use serde::{Deserialize, Serialize};
use std::process::Command;

/// Shell tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellToolConfig {
    pub timeout_seconds: u64,
    pub allowed_commands: Vec<String>,
    pub blocked_commands: Vec<String>,
}

impl Default for ShellToolConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            allowed_commands: vec![],
            blocked_commands: vec![
                "rm -rf /".to_string(),
                "mkfs".to_string(),
                "dd if=/dev/zero".to_string(),
            ],
        }
    }
}

/// Shell command execution tool
pub struct ShellTool {
    config: ShellToolConfig,
}

impl ShellTool {
    /// Create a new ShellTool
    pub fn new(config: ShellToolConfig) -> Self {
        Self { config }
    }

    /// Execute a shell command
    pub async fn execute(&self, command: &str) -> Result<ShellResult, String> {
        // TODO: Implement command validation and timeout
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        Ok(ShellResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        })
    }
}

/// Shell command result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}
