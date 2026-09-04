//! Core configuration for HiTechCloud CLI

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure for HiTechCloud CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiTechCloudConfig {
    /// Default provider to use
    pub default_provider: Option<String>,

    /// Default model to use
    pub default_model: Option<String>,

    /// Provider configurations
    pub providers: Vec<ProviderConfig>,

    /// Agent configuration
    pub agent: AgentConfig,

    /// Session configuration
    pub session: SessionConfig,

    /// Permission configuration
    pub permissions: PermissionConfig,

    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider name (e.g., "openai", "anthropic", "nube")
    pub name: String,

    /// Provider type (e.g., "openai-compatible", "anthropic-compatible")
    pub provider_type: String,

    /// API endpoint URL
    pub endpoint: String,

    /// API key (optional, can be set via environment variable)
    pub api_key: Option<String>,

    /// Default model for this provider
    pub default_model: Option<String>,

    /// Whether this provider is enabled
    pub enabled: bool,
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Maximum tokens per response
    pub max_tokens: Option<u32>,

    /// Temperature for generation
    pub temperature: Option<f32>,

    /// Whether to enable streaming
    pub streaming: bool,

    /// Maximum budget in USD
    pub max_budget_usd: Option<f64>,

    /// Approval mode: "auto", "auto-safe", "always-ask"
    pub approval_mode: String,
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Directory to store session data
    pub data_dir: PathBuf,

    /// Whether to enable WAL mode for crash recovery
    pub wal_mode: bool,

    /// Maximum number of sessions to keep
    pub max_sessions: Option<usize>,
}

/// Permission configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {
    /// Whether to enable secret redaction
    pub secret_redaction: bool,

    /// List of allowed shell commands (empty = all allowed)
    pub allowed_commands: Vec<String>,

    /// List of blocked shell commands
    pub blocked_commands: Vec<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,

    /// Whether to enable debug mode
    pub debug: bool,

    /// Log file path (optional)
    pub file: Option<PathBuf>,
}

impl Default for HiTechCloudConfig {
    fn default() -> Self {
        Self {
            default_provider: Some("nube".to_string()),
            default_model: Some("gpt-4".to_string()),
            providers: vec![],
            agent: AgentConfig::default(),
            session: SessionConfig::default(),
            permissions: PermissionConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_tokens: Some(4096),
            temperature: Some(0.7),
            streaming: true,
            max_budget_usd: None,
            approval_mode: "auto-safe".to_string(),
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        let data_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".hitechcloud")
            .join("sessions");

        Self {
            data_dir,
            wal_mode: true,
            max_sessions: Some(100),
        }
    }
}

impl Default for PermissionConfig {
    fn default() -> Self {
        Self {
            secret_redaction: true,
            allowed_commands: vec![],
            blocked_commands: vec![
                "rm -rf /".to_string(),
                "mkfs".to_string(),
                "dd if=/dev/zero".to_string(),
            ],
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            debug: false,
            file: None,
        }
    }
}

impl HiTechCloudConfig {
    /// Load configuration from file
    pub fn load() -> crate::Result<Self> {
        let config_path = Self::config_path();

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Self = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> crate::Result<()> {
        let config_path = Self::config_path();

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| crate::Error::Config(e.to_string()))?;
        std::fs::write(&config_path, content)?;

        Ok(())
    }

    /// Get the configuration file path
    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".hitechcloud")
            .join("config.toml")
    }

    /// Get a provider configuration by name
    pub fn get_provider(&self, name: &str) -> Option<&ProviderConfig> {
        self.providers.iter().find(|p| p.name == name)
    }

    /// Get the default provider configuration
    pub fn get_default_provider(&self) -> Option<&ProviderConfig> {
        self.default_provider
            .as_ref()
            .and_then(|name| self.get_provider(name))
    }
}
