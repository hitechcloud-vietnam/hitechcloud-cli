//! Permission engine for tool execution approval

use serde::{Deserialize, Serialize};

/// Permission mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionMode {
    /// Automatically approve safe operations
    AutoSafe,
    /// Automatically approve all operations
    Auto,
    /// Always ask for approval
    AlwaysAsk,
}

/// Permission decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionDecision {
    /// Allow the operation
    Allow,
    /// Deny the operation
    Deny,
    /// Ask the user for approval
    Ask,
}

/// Permission engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionEngineConfig {
    pub mode: PermissionMode,
    pub allowed_commands: Vec<String>,
    pub blocked_commands: Vec<String>,
}

impl Default for PermissionEngineConfig {
    fn default() -> Self {
        Self {
            mode: PermissionMode::AutoSafe,
            allowed_commands: vec![],
            blocked_commands: vec![
                "rm -rf /".to_string(),
                "mkfs".to_string(),
                "dd if=/dev/zero".to_string(),
            ],
        }
    }
}

/// Permission engine for evaluating tool execution
pub struct PermissionEngine {
    config: PermissionEngineConfig,
}

impl PermissionEngine {
    /// Create a new PermissionEngine
    pub fn new(config: PermissionEngineConfig) -> Self {
        Self { config }
    }

    /// Evaluate if a command should be allowed
    pub fn evaluate(&self, command: &str) -> PermissionDecision {
        // Check blocked commands first
        for blocked in &self.config.blocked_commands {
            if command.contains(blocked) {
                return PermissionDecision::Deny;
            }
        }

        // Check allowed commands
        if !self.config.allowed_commands.is_empty() {
            let is_allowed = self.config.allowed_commands.iter().any(|a| command.contains(a));
            if !is_allowed {
                return match self.config.mode {
                    PermissionMode::AlwaysAsk => PermissionDecision::Ask,
                    _ => PermissionDecision::Deny,
                };
            }
        }

        // Based on mode
        match self.config.mode {
            PermissionMode::Auto => PermissionDecision::Allow,
            PermissionMode::AutoSafe => {
                // Check if command is potentially dangerous
                if self.is_dangerous(command) {
                    PermissionDecision::Ask
                } else {
                    PermissionDecision::Allow
                }
            }
            PermissionMode::AlwaysAsk => PermissionDecision::Ask,
        }
    }

    /// Check if a command is potentially dangerous
    fn is_dangerous(&self, command: &str) -> bool {
        let dangerous_patterns = [
            "rm ",
            "sudo ",
            "chmod ",
            "chown ",
            "mkfs",
            "dd ",
            "> /dev/",
            "| sudo",
        ];

        dangerous_patterns.iter().any(|p| command.contains(p))
    }
}
