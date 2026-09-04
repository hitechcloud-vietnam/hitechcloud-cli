//! Git operations tool

use serde::{Deserialize, Serialize};

/// Git tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitToolConfig {
    pub auto_commit: bool,
    pub default_branch: String,
}

impl Default for GitToolConfig {
    fn default() -> Self {
        Self {
            auto_commit: false,
            default_branch: "main".to_string(),
        }
    }
}

/// Git operations tool
pub struct GitTool {
    config: GitToolConfig,
}

impl GitTool {
    /// Create a new GitTool
    pub fn new(config: GitToolConfig) -> Self {
        Self { config }
    }

    /// Get git status
    pub async fn status(&self) -> Result<String, String> {
        let output = std::process::Command::new("git")
            .arg("status")
            .output()
            .map_err(|e| format!("Failed to get git status: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Get git diff
    pub async fn diff(&self) -> Result<String, String> {
        let output = std::process::Command::new("git")
            .arg("diff")
            .output()
            .map_err(|e| format!("Failed to get git diff: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Create a git commit
    pub async fn commit(&self, message: &str) -> Result<String, String> {
        let output = std::process::Command::new("git")
            .args(["commit", "-m", message])
            .output()
            .map_err(|e| format!("Failed to create commit: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Create a git branch
    pub async fn branch(&self, name: &str) -> Result<String, String> {
        let output = std::process::Command::new("git")
            .args(["checkout", "-b", name])
            .output()
            .map_err(|e| format!("Failed to create branch: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
