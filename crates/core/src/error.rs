//! Core error types for HiTechCloud CLI

use thiserror::Error;

/// Main error type for HiTechCloud operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("TOML parsing error: {0}")]
    TomlParsing(#[from] toml::de::Error),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Session error: {0}")]
    Session(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Tool execution error: {0}")]
    ToolExecution(String),

    #[error("Agent error: {0}")]
    Agent(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type alias for HiTechCloud operations
pub type Result<T> = std::result::Result<T, Error>;
