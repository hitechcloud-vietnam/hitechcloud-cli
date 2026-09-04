//! Sub-agent definition

use serde::{Deserialize, Serialize};

/// Sub-agent definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAgent {
    pub name: String,
    pub description: String,
    pub system_prompt: String,
    pub tools: Vec<String>,
}

impl SubAgent {
    /// Create a new SubAgent
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        system_prompt: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            system_prompt: system_prompt.into(),
            tools: vec![],
        }
    }
}
